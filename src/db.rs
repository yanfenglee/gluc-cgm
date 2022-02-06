
use mongodb::{Database, options::ClientOptions, Client, Collection};
use once_cell::sync::OnceCell;

use crate::settings::Settings;

// mongodb singleton
static INSTANCE: OnceCell<Database> = OnceCell::new();

pub struct DB;
impl DB {
    pub fn get() -> &'static Database {
        INSTANCE.get().expect("mongo not init!!!")
    }

    pub async fn init() -> Result<(), anyhow::Error>{
            // Parse a connection string into an options struct.
        let mut client_options = ClientOptions::parse(&Settings::get().database_uri).await?;

        // Manually set an option.
        client_options.app_name = Some("gluc-cgm".to_string());

        // Get a handle to the deployment.
        let db = Client::with_options(client_options)?.database(&Settings::get().database_name);
        let _ = INSTANCE.set(db);
        Ok(())
    }

    pub fn coll<T>() -> Collection<T> where T: CollectionName {
        Self::get().collection(T::name())
    }
}

pub trait CollectionName {
    fn name() -> &'static str;
}