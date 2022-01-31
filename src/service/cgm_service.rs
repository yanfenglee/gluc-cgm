use mongodb::{Database, bson::doc};
use anyhow::Result;

use crate::{error::GlucError, MONGO};

///Cgm service
pub struct CgmService {}

impl CgmService {

    pub async fn ping() -> Result<(), GlucError> {
        Ok(())
    }

}
