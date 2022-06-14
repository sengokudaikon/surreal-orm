use common::configurations::redis::RedisConfigs;
use oauth2::basic::BasicClient;
use poem::middleware::AddData;
use poem::web::{Data, Redirect};
use poem::{get, handler, http::Uri, listener::TcpListener, web::Path, Route, Server};
use poem::{EndpointExt, IntoResponse};

// Alternatively, this can be `oauth2::curl::http_client` or a custom client.
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use poem_openapi::payload::{PlainText, Response};
use redis::Connection;
// use redis::aio::Connection;
use std::env;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
// use tokio::net::TcpListener;
use crate::oauth::github::GithubConfig;
use crate::oauth::utils::{OauthProviderTrait, TypedAuthUrl, TypedCsrfState};
use url::Url;

use crate::app::user::{OauthProvider, User};

#[handler]
pub async fn oauth_login(Path(provider): Path<OauthProvider>, rc: Data<&RedisConfigs>) -> Redirect {
    let mut con = rc.clone().get_client().unwrap().get_connection().unwrap();

    let auth_url_data = match provider {
        OauthProvider::Github => GithubConfig::new().generate_auth_url(),
        OauthProvider::Google => todo!(),
    };

    // Send csrf state to redis
    auth_url_data.csrf_state.cache(provider, &mut con).unwrap();

    Redirect::moved_permanent(auth_url_data.authorize_url)
}

#[handler]
async fn oauth_redirect_url(uri: &Uri, rc: Data<&RedisConfigs>) -> String {
    let redirect_url = Url::parse(&("http://localhost".to_string() + &uri.to_string())).unwrap();
    let redirect_url = TypedAuthUrl(redirect_url);
    let code = redirect_url.get_authorization_code();

    let mut con = rc.clone().get_client().unwrap().get_connection().unwrap();
    // make .verify give me back both the csrf token and the provider
    let provider = redirect_url.get_csrf_state().verify(&mut con).expect("er");

    let user = match provider {
        OauthProvider::Github => {
            let github_config = GithubConfig::new();
            println!("my state: {provider:?}");

            // All these are the profile fetch should probably also be part of github config(OauthProvider) trait
            github_config.fetch_oauth_account(code).await.unwrap()
        }
        OauthProvider::Google => todo!(),
    };

    //  Also, handle storing user session
    // poem::Response::builder().body(user).finish()
    "efddfd".into()
}
