
use anyhow::Result;

///Cgm service
pub struct CgmService;

impl CgmService {

    pub async fn ping() -> Result<(), anyhow::Error> {
        Ok(())
    }

}
