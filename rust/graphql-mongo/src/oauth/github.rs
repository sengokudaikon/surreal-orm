use chrono::{Duration, Utc};
use common::configurations::oauth::OauthGithubCredentials;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, PkceCodeVerifier, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};

use super::utils::{get_redirect_url, OauthConfig, OauthProviderTrait, OauthResult, OauthUrl};
use crate::app::user::{AccountOauth, OauthProvider, TokenType};

#[derive(Debug, Deserialize, Serialize)]
struct GithubUserData {
    id: u32,
    login: String,
    name: Option<String>,
    email: Option<String>,
    avatar_url: Option<String>,
    gravatar_id: Option<String>,
    url: Option<String>,
    location: Option<String>,
    // Many other irrelevant fields discarded
}

#[derive(Debug, Deserialize)]
struct GithubEmail {
    email: String,
    primary: bool,
    verified: bool,
    // visibility: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct GithubConfig {
    pub(crate) basic_config: OauthConfig,
}

impl GithubConfig {
    pub fn new() -> Self {
        let env = OauthGithubCredentials::default();
        let basic_config = OauthConfig {
            client_id: ClientId::new(env.client_id),
            client_secret: ClientSecret::new(env.client_secret),
            auth_url: AuthUrl::new("https://github.com/login/oauth/authorize".to_string())
                .expect("Invalid authorization endpoint URL"),
            token_url: TokenUrl::new("https://github.com/login/oauth/access_token".to_string())
                .expect("Invalid token endpoint URL"),
            redirect_url: RedirectUrl::new(get_redirect_url()).expect("Invalid redirect URL"),
            scopes: vec![
                Scope::new("public_repo".into()),
                Scope::new("read:user".into()),
                Scope::new("user:email".into()),
            ],
            provider: OauthProvider::Github,
            revocation_url: None,
        };
        Self { basic_config }
    }
}

#[async_trait::async_trait]
impl OauthProviderTrait for GithubConfig {
    fn basic_config(&self) -> OauthConfig {
        self.basic_config.to_owned()
    }

    async fn fetch_oauth_account(
        &self,
        code: AuthorizationCode,
        pkce_code_verifier: PkceCodeVerifier,
    ) -> OauthResult<AccountOauth> {
        let token = self.exchange_token(code, pkce_code_verifier).await?;

        let profile = OauthUrl("https://api.github.com/user")
            .fetch_resource::<GithubUserData>(&token, None)
            .await?;

        let user_emails = OauthUrl("https://api.github.com/user/emails")
            .fetch_resource::<Vec<GithubEmail>>(&token, None)
            .await?;

        let expiration = token.expires_in().unwrap_or(std::time::Duration::new(0, 0));
        let expiration = Duration::from_std(expiration).unwrap_or_else(|_| Duration::seconds(0));
        let expires_at = Utc::now() + expiration;
        let scopes = self
            .basic_config()
            .scopes
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        // Get the primary email or any first
        let primary_email = user_emails
            .iter()
            .find(|r| r.primary)
            .or_else(|| user_emails.first());

        let email = primary_email.map(|p| p.email.to_string());
        let account = AccountOauth::builder()
            .id(profile.id.to_string())
            .display_name(Some(profile.login.clone()))
            .provider(OauthProvider::Github)
            .provider_account_id(OauthProvider::Github)
            .access_token(token.access_token().secret().into())
            .refresh_token(token.refresh_token().map(|rf| rf.secret().into()))
            .expires_at(Some(expires_at))
            .token_type(Some(TokenType::Bearer))
            .scopes(scopes)
            .email(email.or(profile.email))
            .email_verified(primary_email.map(|p| p.verified).unwrap_or(false))
            .build();

        Ok(account)
    }
}
