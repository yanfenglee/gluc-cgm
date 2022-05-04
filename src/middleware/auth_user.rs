
use axum::{async_trait, extract::{FromRequest, RequestParts}};
use mongodb::bson::{doc};

use crate::{structs::User, DB, error::GlucError};


pub async fn get_user_from_token(token: &String) -> Option<User> {
    let user = DB::coll::<User>()
        .find_one(doc! {"token":token}, None)
        .await
        .ok()??;

    tracing::debug!("token to user: {:?}", user);

    Some(user)
}


#[async_trait]
impl<B> FromRequest<B> for User
where
    B: Send,
{
    type Rejection = GlucError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {

        let header = req.headers().get("token").ok_or_else(|| GlucError::AuthError("no token found".to_owned()))?;

        let token = header.to_str()?.to_string();

        let user = get_user_from_token(&token).await.ok_or_else(|| GlucError::AuthError("auth failed".to_owned()))?;

        Ok(user)
        
    }
}

