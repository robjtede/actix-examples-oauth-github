//! Sample app demonstrating GitHub OAuth login using Actix Web.

use actix_web::{middleware::Logger, web, Scope};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_secrets::SecretStore;

mod routes;

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> ShuttleActixWeb<impl FnOnce(&mut web::ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut web::ServiceConfig| {
        cfg.service(
            Scope::new("")
                .app_data(web::Data::new(secret_store))
                .service(routes::index)
                .service(routes::auth_github_callback)
                .wrap(Logger::default()),
        );
    };

    Ok(config.into())
}
