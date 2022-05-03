use std::{env, str::FromStr};

use config::{Config, ConfigError, Environment, File};
use once_cell::sync::OnceCell;
use serde::Deserialize;
use tracing::{info, Level};
use tracing_subscriber::{self, fmt::time};

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub database_uri: String,
    pub database_name: String,
    pub bind_addr: String,
    pub log_level: String,
}

static INSTANCE: OnceCell<Settings> = OnceCell::new();

impl Settings {
    pub fn init() -> Result<(), ConfigError> {
        let env = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name(&format!("config/{}", env)).required(false))
            .add_source(File::with_name("config/local").required(false))
            .add_source(Environment::with_prefix("gluc")).build()?;

        // You can deserialize (and thus freeze) the entire configuration as
        let setting: Settings = s.try_deserialize()?;

        Self::setup_log(&setting.log_level);

        info!("settings: {:?}", setting);

        INSTANCE.set(setting).unwrap();
        Ok(())
    }


    fn setup_log(level: &String) {
        let level = Level::from_str(level).unwrap();
        tracing_subscriber::fmt()
            .with_max_level(level)
            //.with_timer(time::ChronoLocal::with_format(String::from("%Y-%m-%d %H:%M:%S%.6f")))
            .with_timer(time::ChronoLocal::rfc3339())
            .init();
    }

    
    pub fn get() -> &'static Self {
        INSTANCE.get().expect("config not init!!!")
    }
}
