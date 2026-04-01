//! Sample app demonstrating GitHub OAuth login using Actix Web.

mod config;
mod init;
mod routes;

use actix_web::{
    App, HttpServer,
    middleware::{Compress, Logger, NormalizePath},
    web,
};
use eyre::WrapErr as _;

use crate::config::AppConfig;

#[actix_web::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    dotenvy::dotenv().ok();

    crate::init::tracing()?;

    let config = AppConfig::init()?;

    let bind_host = config.host.clone();
    let bind_port = config.port;

    HttpServer::new({
        let config = web::Data::new(config.clone());
        move || {
            App::new()
                .app_data(config.clone())
                .service(routes::index)
                .service(routes::healthz)
                .service(routes::auth_github_callback)
                .wrap(Compress::default())
                .wrap(NormalizePath::trim())
                .wrap(Logger::default().log_target("@"))
        }
    })
    .workers(2)
    .bind((bind_host.as_str(), bind_port))
    .wrap_err_with(|| format!("Failed to bind HTTP server to {bind_host}:{bind_port}"))?
    .run()
    .await
    .wrap_err("HTTP server exited with an error")?;

    Ok(())
}
