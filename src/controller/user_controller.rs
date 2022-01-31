use actix_web::{post, web};
use mongodb::bson::doc;

use crate::{
    error::GlucError,
    ret,
    structs::{User, UserDTO, UserLoginDTO, UserRegisterDTO},
    util::hash,
    Ret, MONGO,
};

/// config route service
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/user").service(register).service(login));
}

#[post("/register")]
pub async fn register(arg: web::Json<UserRegisterDTO>) -> Result<Ret<()>, GlucError> {
    let db = MONGO.get().unwrap();

    let user = User {
        username: arg.username.clone(),
        password: Some(arg.password.clone()),
        nickname: arg.nickname.clone(),
        email: arg.email.clone(),
        phone: arg.phone.clone(),
        token: hash::sha1(&format!("{}_{}", arg.username, arg.password)),
    };

    db.collection("cgm").insert_one(user, None).await?;

    ret(())
}

#[post("/login")]
pub async fn login(arg: web::Json<UserLoginDTO>) -> Result<Ret<UserDTO>, GlucError> {
    log::info!("user login: {:?}", arg);
    let db = MONGO.get().unwrap();

    if let Some(user) = db
        .collection::<User>("cgm")
        .find_one(
            doc! {"username":arg.username.clone(),"password":arg.password.clone()},
            None,
        )
        .await?
    {
        return ret(UserDTO {
            token: user.token,
            username: user.username,
            nickname: user.nickname,
        });
    } else {
        return Err(GlucError::AuthError("用户名或密码错误".to_owned()));
    };
}
