use axum::{http::StatusCode, Extension};
use diesel::prelude::*;
use std::sync::Arc;

use crate::db::{establish_connection, models::User, schema::users};

pub async fn delete_user(Extension(user): Extension<Arc<User>>) -> StatusCode {
    let mut conn = establish_connection();
    match diesel::delete(users::table.filter(users::id.eq(user.id))).execute(&mut conn) {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
