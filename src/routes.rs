//! Web server route handlers.

use actix_web::{Responder, get, web};
use maud::{Markup, html};
use octocrab::Octocrab;
use secrecy::ExposeSecret as _;
use serde::Deserialize;

use crate::config::AppConfig;

#[get("/")]
pub async fn index(config: web::Data<AppConfig>) -> impl Responder {
    let client_id = &config.gh_client_id;
    let redirect_uri = github_callback_url(config.get_ref());

    wrap_body(html! {
        a href=(format!(
            "https://github.com/login/oauth/authorize?client_id={client_id}&redirect_uri={redirect_uri}"
        )) {
            "Login with GitHub"
        }
    })
}

#[derive(Debug, Deserialize)]
pub struct CallbackParams {
    code: String,
}

#[get("/auth/github/callback")]
pub async fn auth_github_callback(
    config: web::Data<AppConfig>,
    web::Query(params): web::Query<CallbackParams>,
) -> impl Responder {
    let client_id = &config.gh_client_id;
    let client_secret = config.gh_client_secret.expose_secret();
    let redirect_uri = github_callback_url(config.get_ref());

    let oauth_client = octocrab::Octocrab::builder()
        .base_uri("https://github.com")
        .unwrap()
        .add_header("accept".parse().unwrap(), "application/json".to_string())
        .build()
        .unwrap();

    let oauth = oauth_client
        .post::<_, serde_json::Value>(
            "/login/oauth/access_token",
            Some(&serde_json::json!({
                "code": params.code,
                "client_id": client_id,
                "client_secret": client_secret,
                "redirect_uri": redirect_uri,
            })),
        )
        .await
        .unwrap();

    let oauth = serde_json::from_value::<octocrab::auth::OAuth>(oauth.clone())
        .unwrap_or_else(|_| panic!("couldn't parse OAuth credentials from {oauth:?}"));

    let client = Octocrab::builder()
        .user_access_token(oauth.access_token.expose_secret())
        .build()
        .unwrap();

    let user = client.current().user().await.unwrap();

    wrap_body(html! {
        (format!("Hello, {}!", user.login))
    })
}

fn wrap_body(markup: Markup) -> Markup {
    html! {
        (maud::DOCTYPE);

        html lang="en" {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0" ;
                title { "GitHub Login Example" }
            }

            body { (markup) }
        }
    }
}

fn github_callback_url(config: &AppConfig) -> String {
    format!(
        "{}/auth/github/callback",
        config.public_base_url.trim_end_matches('/')
    )
}
