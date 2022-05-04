use std::{env, str::FromStr};

use config::{Config, ConfigError, Environment, File};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use tracing::{Level};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{self, fmt::time};

#[derive(Debug, Serialize, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub mode: String,
    pub database_uri: String,
    pub database_name: String,
    pub bind_addr: String,
    pub log_level: String,
    pub log_path: String,

    #[serde(skip)]
    _log_guard: Option<WorkerGuard>,
}

static INSTANCE: OnceCell<Settings> = OnceCell::new();

impl Settings {
    pub fn init() -> Result<(), ConfigError> {
        let env = env::var("RUN_MODE").unwrap_or_else(|_| "dev".into());

        let s = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name(&format!("config/{}", env)).required(false))
            .add_source(Environment::with_prefix("gluc")).build()?;

        // You can deserialize (and thus freeze) the entire configuration as
        let mut setting: Settings = s.try_deserialize()?;

        setting._log_guard = Some(Self::setup_log(&setting));

        tracing::info!("settings: {:?}", setting);

        INSTANCE.set(setting).unwrap();
        Ok(())
    }


    fn setup_log(settings: &Settings) -> WorkerGuard {
        let level = Level::from_str(&settings.log_level).unwrap();

        let (writer, guard) = if settings.mode == "dev" {
            tracing_appender::non_blocking(std::io::stdout())
        } else {
            let file_appender = tracing_appender::rolling::daily(&settings.log_path, "gluc.log");
            tracing_appender::non_blocking(file_appender)
        };

        tracing_subscriber::fmt()
            .with_max_level(level)
            .with_timer(time::SystemTime)
            .with_writer(writer)
            .init();

        return guard;
    }

    
    pub fn get() -> &'static Self {
        INSTANCE.get().expect("config not init!!!")
    }
}
