use axum::{Json, Router, routing::{post, get}};
use mongodb::bson::{doc, oid::ObjectId, DateTime};

use crate::{
    error::GlucError,
    ret,
    structs::{User, UserDTO, UserLoginDTO, UserRegisterDTO},
    util::hash,
    Ret, DB,
};


pub fn route() -> Router {
    let app = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/check", get(check));

    app
}


pub async fn register(Json(arg): Json<UserRegisterDTO>) -> Result<Ret<()>, GlucError> {

    tracing::info!("begin login: {:?}", arg);

    if let Some(_user) = DB::coll::<User>().find_one(doc!{"username": arg.username.clone()}, None).await? {
        return Err(GlucError::RegisterError(format!("用户名已存在!!! {}", _user.username)));
    }

    let hs = hash::sha1(&format!("{}_{}", arg.username, arg.password));

    let user = User {
        _id: ObjectId::new(),
        username: arg.username.clone(),
        password: Some(arg.password.clone()),
        nickname: arg.nickname.clone(),
        email: arg.email.clone(),
        phone: arg.phone.clone(),
        token: hs,
        create_time: DateTime::now(),
    };

    DB::coll().insert_one(user, None).await?;

    ret(())
}

pub async fn login(Json(arg): Json<UserLoginDTO>) -> Result<Ret<UserDTO>, GlucError> {

    tracing::info!("login called {:?}", arg);

    if let Some(user) = DB::coll::<User>()
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


pub async fn check(user: Option<User>) -> Result<Ret<String>, GlucError> {
    if user.is_some() {
        return ret("user.user".into());
    } else {
        return Err(GlucError::AuthError("auth failed222".to_owned()));
    }
}
