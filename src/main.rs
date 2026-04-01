//! Sample app demonstrating GitHub OAuth login using Actix Web.

mod config;
mod routes;

use actix_web::{
    App, HttpServer,
    middleware::{Compress, Logger, NormalizePath},
    web,
};
use confik::{Configuration, EnvSource};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt as _, util::SubscriberInitExt as _};

use crate::config::AppConfig;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_tracing();

    let config = AppConfig::builder()
        .override_with(EnvSource::new().allow_secrets())
        .try_build()
        .map_err(|err| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("failed to load application configuration: {err}"),
            )
        })?;

    let bind_host = config.host.clone();
    let bind_port = config.port;
    let state = web::Data::new(config);

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(routes::index)
            .service(routes::auth_github_callback)
            .wrap(Compress::default())
            .wrap(NormalizePath::trim())
            .wrap(Logger::default())
    })
    .bind((bind_host.as_str(), bind_port))?
    .run()
    .await
}

fn init_tracing() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer())
        .init();
}
