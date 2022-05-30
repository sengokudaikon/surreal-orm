use anyhow::Context as ContextAnyhow;
use bson::oid::ObjectId;
use common::{
    authentication::{
        TypedSession, {generate_password_hash, validate_password, PasswordHashPHC, PasswordPlain},
    },
    error_handling::ApiHttpStatus,
};
use log::{error, warn};

use super::{AccountOauth, Role, SignInCredentials, SignOutMessage, User};
use async_graphql::*;
use chrono::Utc;

use mongodb::Database;
use validator::Validate;
use wither::Model;

#[derive(Default)]
pub struct UserMutationRoot;

#[Object]
impl UserMutationRoot {
    async fn create_user(
        &self,
        ctx: &async_graphql::Context<'_>,
        #[graphql(desc = "user data")] user_input: User,
    ) -> anyhow::Result<User> {
        user_input.validate()?;
        let db = ctx.data_unchecked::<Database>();

        let mut user = User::builder()
            .created_at(Utc::now())
            .username(user_input.username)
            .first_name(user_input.first_name)
            .last_name(user_input.last_name)
            .email(user_input.email)
            .email_verified_at(None)
            .age(user_input.age)
            .social_media(user_input.social_media)
            .roles(vec![Role::User])
            .password(user_input.password)
            .accounts(vec![])
            .build();

        user.save(db, None).await?;

        Ok(user)
    }

    /// Creates a new user but doesn't log in the user
    /// Currently like this because of future developments
    async fn sign_up(
        &self,
        ctx: &async_graphql::Context<'_>,
        #[graphql(desc = "Sign Up credentials")] user: User,
    ) -> FieldResult<User> {
        user.validate()?;

        let db = ctx.data_unchecked::<Database>();
        let password_hash = generate_password_hash(user.password.context("Invalid password")?)
            .await
            .map_err(|_| ApiHttpStatus::BadRequest("Password badly formed".into()))?;

        let mut user = User::builder()
            .created_at(Utc::now())
            .username(user.username)
            .first_name(user.first_name)
            .last_name(user.last_name)
            .email(user.email)
            .email_verified_at(None)
            .age(user.age)
            .social_media(user.social_media)
            .roles(vec![Role::User])
            .accounts(vec![])
            .password(Some(password_hash.into()))
            .build();

        user.save(db, None).await.map_err(|e| {
            error!("{:?}", e.to_string());
            ApiHttpStatus::BadRequest("Unable to save your data. Try again later".into()).extend()
        })?;
        Ok(user)
    }

    async fn sign_in(
        &self,
        ctx: &async_graphql::Context<'_>,
        #[graphql(desc = "sign in credentials")] sign_in_credentials: SignInCredentials,
    ) -> FieldResult<User> {
        let db = ctx.data::<Database>()?;
        let session = TypedSession::from_ctx(ctx)?;
        let maybe_user_id = session.get_user_id::<ObjectId>().ok();

        // Return user if found from session
        let user = match maybe_user_id {
            Some(ref user_id) => {
                let user = User::find_by_id(db, user_id).await;
                session.renew();
                user
            }
            // If not found from session, handle fresh signin flow
            None => {
                let user = User::find_by_username(db, sign_in_credentials.username).await?;

                let password_hash = &user.password.clone().ok_or_else(|| {
                    error!("Password does not exist for normal signed in user 🤔");
                    ApiHttpStatus::Unauthorized("Invalid Credentials".into()).extend()
                })?;

                let plain_password = PasswordPlain::new(sign_in_credentials.password);
                let hashed_password = PasswordHashPHC::new(password_hash);
                validate_password(plain_password, hashed_password).await?;

                let id = user.id.ok_or_else(|| {
                    ApiHttpStatus::InternalServerError("Malformed id".into()).extend()
                })?;

                session.insert_user_id(&id).map_err(|e| {
                    warn!("{e:?}");
                    ApiHttpStatus::InternalServerError("Error handling us".into()).extend()
                })?;
                Ok(user)
            }
        };

        user
    }

    // TODO: Improve all errors using error extension
    async fn create_or_update_user_oauth(
        &self,
        ctx: &async_graphql::Context<'_>,
        #[graphql(desc = "user account credentials")] account: AccountOauth,
    ) -> FieldResult<User> {
        // TODO: Limit this call to only server. Our nextjs server will call this during oauth flow and the relay
        // our cookie session to the client
        // let was_in_headers = ctx.insert_http_header(ACCESS_CONTROL_ALLOW_ORIGIN, "*");

        let db = ctx.data_unchecked::<Database>();
        let session = ctx.data::<TypedSession>()?;

        // ALREADY LOGGED IN OAUTH USER
        let maybe_user_id = session.get_user_id::<ObjectId>().ok();

        // Return user if found from session
        let user = match maybe_user_id {
            Some(ref user_id) => {
                let user = User::find_by_id(db, user_id).await;
                // Found from session, so, renew
                session.renew();
                user
            }

            None => {
                // This should upsert user based on if they have been created by their oauth credentials or not.
                // If they already have an id based on the filter, it will update their data, otherwise, it will
                // create a new record
                let account_clone = account.clone();
                let provider = account_clone.provider.as_str();
                let provider_account_id = account_clone.provider_account_id.as_str();
                let user = User::builder()
                    .created_at(Utc::now())
                    .username(format!("{}-{}", provider, provider_account_id))
                    .first_name(account_clone.profile.first_name)
                    .last_name(account_clone.profile.last_name)
                    .email(account_clone.profile.email)
                    .social_media(vec![])
                    .roles(vec![Role::User])
                    .age(None)
                    .accounts(vec![account])
                    .email_verified_at(None)
                    .password(None)
                    .build();

                let user = user
                    .find_or_replace_account_oauth(db, provider, provider_account_id)
                    .await
                    .map_err(|_| {
                        ApiHttpStatus::Unauthorized("Invalid credentials".into()).extend()
                    })?;

                let user_id = user
                    .id(ctx)
                    .await?
                    .ok_or(ApiHttpStatus::Unauthorized("Invalid credentials".into()).extend())?;

                session.insert_user_id(&user_id)?;
                Ok(user)
            }
        };

        user
    }

    async fn sign_out(&self, ctx: &async_graphql::Context<'_>) -> FieldResult<SignOutMessage> {
        let session = TypedSession::from_ctx(ctx)?;
        let user_id = session.get_user_id()?;

        session.clear();
        Ok(SignOutMessage {
            message: "Successfully signed out".into(),
            user_id,
        })
    }
}
