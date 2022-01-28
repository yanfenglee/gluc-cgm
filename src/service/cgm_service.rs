use mongodb::{Client, options::ClientOptions};
use anyhow::Result;
///Cgm service
pub struct CgmService {}

impl CgmService {
    // pub async fn add(&self, arg: &Vec<BgDTO>, user_id: i64) -> Result<u64> {
    //     let entries: Vec<Cgm> = arg.iter().map(|item| {
    //         let mut cgm: Cgm = item.into();
    //         cgm.id = Some(rbatis::plugin::snowflake::new_snowflake_id());
    //         cgm.user_id = Some(user_id);
    //         cgm
    //     }).collect();

    //     Ok(RB.save_batch(&entries, &[]).await?.rows_affected)
    // }


    // pub async fn list(&self, ts: i64, cnt: i64, user_id: i64) -> Result<Vec<BgDTO>> {

    //     #[sql(RB, "SELECT * FROM cgm WHERE user_id = ? and `date` < ? order by `date` desc LIMIT ?")]
    //     async fn select_entries(user_id: i64, ts: i64, cc: i64) -> Vec<Cgm> {}

    //     let cgms = select_entries(user_id, ts, cnt).await?;

    //     Ok(cgms.iter().map(|x| x.into()).collect())
    // }

    pub async fn ping() -> Result<()> {
        // Parse a connection string into an options struct.
        let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;

        // Manually set an option.
        client_options.app_name = Some("My App".to_string());

        // Get a handle to the deployment.
        let client = Client::with_options(client_options)?;

        // List the names of the databases in that deployment.
        for db_name in client.list_database_names(None, None).await? {
            println!("{}", db_name);
        }

        Ok(())
        
    }
}
