use actix_web::{get, http::header::ACCEPT, web, Responder};
use actix_web_lab::respond::Html;
use octocrab::Octocrab;
use secrecy::ExposeSecret as _;
use serde::Deserialize;
use shuttle_secrets::SecretStore;

#[get("/")]
pub async fn index(secrets: web::Data<SecretStore>) -> impl Responder {
    let client_id = secrets.get("gh_client_id").unwrap();

    Html::new(format!(
        r#"<a href="https://github.com/login/oauth/authorize?client_id={client_id}"
    Login with GitHub
</a>"#
    ))
}

#[derive(Debug, Deserialize)]
pub struct CallbackParams {
    code: String,
}

#[get("/auth/github/callback")]
pub async fn auth_github_callback(
    secrets: web::Data<SecretStore>,
    web::Query(params): web::Query<CallbackParams>,
) -> impl Responder {
    println!("code = {}", &params.code);

    let client_id = secrets.get("gh_client_id").unwrap();
    let client_secret = secrets.get("gh_client_secret").unwrap();

    let oauth_client = octocrab::Octocrab::builder()
        .base_uri("https://github.com")
        .unwrap()
        .add_header(ACCEPT, "application/json".to_string())
        .build()
        .unwrap();

    let oauth = oauth_client
        .post::<_, serde_json::Value>(
            "/login/oauth/access_token",
            Some(&serde_json::json!({
                "code": params.code,
                "client_id": client_id,
                "client_secret": client_secret,
            })),
        )
        .await
        .unwrap();

    let oauth = serde_json::from_value::<octocrab::auth::OAuth>(oauth.clone())
        .unwrap_or_else(|_| panic!("couldn't parse OAuth credentials from {oauth:?}"));

    let client = Octocrab::builder()
        .user_access_token(oauth.access_token.expose_secret().clone())
        .build()
        .unwrap();

    let user = client.current().user().await.unwrap();

    web::Json(serde_json::json!({
        "login": user.login,
    }))
}
