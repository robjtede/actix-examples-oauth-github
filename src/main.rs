//! Sample app demonstrating GitHub OAuth login using Actix Web.

use actix_web::{
    middleware::{Compress, Logger, NormalizePath},
    web, Scope,
};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::SecretStore;

mod routes;

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
) -> ShuttleActixWeb<impl FnOnce(&mut web::ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut web::ServiceConfig| {
        cfg.service(
            Scope::new("")
                .app_data(web::Data::new(secret_store))
                .service(routes::index)
                .service(routes::auth_github_callback)
                .wrap(Compress::default())
                .wrap(NormalizePath::trim())
                .wrap(Logger::default()),
        );
    };

    Ok(config.into())
}
