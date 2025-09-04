use config::{Config, Environment, File};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Settings {
    pub database_url: String,
    pub server_port: u16,
    pub api_key: String,
    pub log_level: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            database_url: "sqlite:database.db?mode=rwc".to_string(),
            server_port: 8080,
            api_key: "very-secret-key!!!".to_string(),
            log_level: "info".to_string(),
        }
    }
}

fn ensure_config_exists(path: &str) {
    let path_obj = Path::new(path);

    if let Some(parent) = path_obj.parent()
        && !parent.exists()
    {
        fs::create_dir_all(parent).expect("Failed to create config directory");
    }

    if !path_obj.exists() {
        println!("Config file does not exist: {}", path);
        println!("Creating default at: {}", path);

        let default = Settings::default();

        let toml_str =
            toml::to_string_pretty(&default).expect("Could not serialize default configuration");

        fs::write(path, toml_str).expect("Failed to write default configuration");
    }
}

pub fn load_settings() -> Settings {
    let path = "config/config.toml";
    ensure_config_exists(path);

    let cfg = Config::builder()
        .add_source(File::with_name(path))
        .add_source(
            Environment::with_prefix("TOME")
                .keep_prefix(false)
        )
        .build()
        .expect("Failed to build configuration");

    cfg.try_deserialize::<Settings>()
        .expect("Failed to deserialize configuration")
}
