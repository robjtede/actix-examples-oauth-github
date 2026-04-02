//! Application configuration.

use confik::{Configuration, EnvSource};
use eyre::{Result, WrapErr as _};
use secrecy::SecretString;

#[derive(Clone, Debug, Configuration)]
pub struct AppConfig {
    #[confik(default = "0.0.0.0")]
    pub host: String,

    #[confik(default = 8080_u16)]
    pub port: u16,

    #[confik(default = "http://localhost:8080")]
    pub public_base_url: String,

    pub gh_client_id: String,

    #[confik(secret)]
    pub gh_client_secret: SecretString,
}

impl AppConfig {
    pub fn init() -> Result<Self> {
        let config = Self::builder()
            .override_with(EnvSource::new().allow_secrets())
            .try_build()
            .wrap_err("Failed to load application configuration from environment");

        tracing::info!("Loaded configuration: {config:?}");

        config
    }
}
