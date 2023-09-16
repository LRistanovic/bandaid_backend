use crate::db::models::*;
use axum::{http::StatusCode, Json};
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};

use crate::db::{
    establish_connection,
    models::{City, Country},
    schema::{cities, countries},
};

pub async fn list_cities() -> Result<Json<Vec<ResCity>>, StatusCode> {
    let mut conn = establish_connection();

    let results = cities::table
        .inner_join(countries::table)
        .select((City::as_select(), Country::as_select()))
        .load::<(City, Country)>(&mut conn)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|(city, country)| ResCity {
            id: city.id,
            name: city.name,
            country,
        })
        .collect();

    Ok(Json(results))
}
