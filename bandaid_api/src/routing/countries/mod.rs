use axum::{http::StatusCode, Json};
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};

use crate::db::{establish_connection, models::Country, schema::countries};

pub async fn list_countries() -> Result<Json<Vec<Country>>, StatusCode> {
    let mut conn = establish_connection();

    let results = countries::table
        .select(Country::as_select())
        .load(&mut conn)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(results))
}
