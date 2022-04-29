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
        let mut s = Config::new();

        // Start off by merging in the "default" configuration file
        s.merge(File::with_name("config/default"))?;

        // Add in the current environment file
        // Default to 'development' env
        // Note that this file is _optional_
        let env = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        s.merge(File::with_name(&format!("config/{}", env)).required(false))?;

        // Add in a local configuration file
        // This file shouldn't be checked in to git
        s.merge(File::with_name("config/local").required(false))?;

        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        s.merge(Environment::with_prefix("gluc"))?;

        // You can deserialize (and thus freeze) the entire configuration as
        let setting: Settings = s.try_into()?;

        // init log
        let level = Level::from_str(setting.log_level.as_str()).unwrap();
        tracing_subscriber::fmt()
            .with_max_level(level)
            //.with_timer(time::ChronoLocal::with_format(String::from("%Y-%m-%d %H:%M:%S%.6f")))
            .with_timer(time::ChronoLocal::rfc3339())
            .init();

        info!("settings: {:?}", setting);

        INSTANCE.set(setting).unwrap();
        Ok(())
    }

    pub fn get() -> &'static Self {
        INSTANCE.get().expect("config not init!!!")
    }
}
