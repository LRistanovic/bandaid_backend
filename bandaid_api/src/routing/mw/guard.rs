use std::sync::Arc;

use crate::db::{establish_connection, models::User, schema::users};

use super::jwt::validate;
use axum::{
    headers::{authorization::Bearer, Authorization, HeaderMapExt},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use diesel::prelude::*;

pub async fn guard<T>(mut request: Request<T>, next: Next<T>) -> Result<Response, StatusCode> {
    let token = request
        .headers()
        .typed_get::<Authorization<Bearer>>()
        .ok_or(StatusCode::UNAUTHORIZED)?
        .token()
        .to_owned();
    let user_id = validate(&token)?;

    let mut conn = establish_connection();

    let user: User = users::table
        .find(user_id)
        .select(User::as_select())
        .load::<User>(&mut conn)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .next()
        .ok_or(StatusCode::UNAUTHORIZED)?;

    request.extensions_mut().insert(Arc::new(user));

    Ok(next.run(request).await)
}
