
use mongodb::{Database, options::ClientOptions, Client, Collection};
use once_cell::sync::OnceCell;

// mongodb singleton
static INSTANCE: OnceCell<Database> = OnceCell::new();

pub struct DB;
impl DB {
    pub fn get() -> &'static Database {
        INSTANCE.get().expect("mongo not init!!!")
    }

    pub async fn init() -> Result<(), anyhow::Error>{
            // Parse a connection string into an options struct.
        let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;

        // Manually set an option.
        client_options.app_name = Some("My App".to_string());

        // Get a handle to the deployment.
        let db = Client::with_options(client_options)?.database("cgm");
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