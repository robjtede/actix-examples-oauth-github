//! Application configuration.

use confik::Configuration;
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
