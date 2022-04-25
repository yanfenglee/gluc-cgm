use std::env;

use config::{File, Environment, Config, ConfigError};
use tracing::info;
use once_cell::sync::OnceCell;
use serde::Deserialize;


#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub database_uri: String,
    pub database_name: String,
    pub bind_addr: String,
}

static INSTANCE: OnceCell<Settings> = OnceCell::new();

impl Settings {
    pub fn init() -> Result<(), ConfigError>{
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
        let setting = s.try_into()?;
        info!("settings: {:?}", setting);
        
        INSTANCE.set(setting).unwrap();
        Ok(())
    }

    pub fn get() -> &'static Self {
        INSTANCE.get().expect("config not init!!!")
    }
}
