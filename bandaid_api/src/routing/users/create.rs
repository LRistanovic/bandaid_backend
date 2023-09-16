use axum::{http::StatusCode, Json};
use bcrypt::*;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::db::{
    establish_connection,
    models::{City, Country, NewUser, ResCity, ResUser, User},
    schema::{cities, countries, users},
};

pub async fn create_user(Json(mut new_user): Json<NewUser>) -> Result<Json<ResUser>, StatusCode> {
    let mut conn = establish_connection();

    new_user.password = hash(new_user.password, DEFAULT_COST).map_err(|err| {
        println!("{err}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let user: User = diesel::insert_into(users::table)
        .values(new_user)
        .returning(User::as_returning())
        .get_result(&mut conn)
        .map_err(|err| {
            println!("{err}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let result = users::table
        .inner_join(cities::table.inner_join(countries::table))
        .filter(users::id.eq(user.id))
        .select((User::as_select(), City::as_select(), Country::as_select()))
        .load::<(User, City, Country)>(&mut conn)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|(user, city, country)| ResUser {
            id: user.id,
            username: user.username,
            first_name: user.first_name,
            last_name: user.last_name,
            description: user.description,
            birth_year: user.birth_year,
            city: ResCity {
                id: city.id,
                name: city.name,
                country,
            },
            profile_picture_url: user.profile_picture_url,
        })
        .next()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(result))
}
