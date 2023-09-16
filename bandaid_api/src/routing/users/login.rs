use axum::{http::StatusCode, Json};
use bcrypt::verify;
use diesel::prelude::*;

use crate::{
    db::{
        establish_connection,
        models::{LoginCredentials, Token, User},
        schema::users,
    },
    routing::mw::jwt::create_jwt,
};

pub async fn login_user(
    Json(credentials): Json<LoginCredentials>,
) -> Result<Json<Token>, StatusCode> {
    let mut conn = establish_connection();

    let user = users::table
        .filter(users::username.eq(credentials.username))
        .select(User::as_select())
        .load::<User>(&mut conn)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .next()
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let valid = verify(&credentials.password, &user.password).map_err(|err| {
        println!("{}", err);
        StatusCode::IM_A_TEAPOT
    })?;
    if !valid {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(Json(Token {
        token: create_jwt(&user.id)?,
    }))
}
