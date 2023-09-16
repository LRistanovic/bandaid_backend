use crate::db::{establish_connection, models::*, schema::countries};
use axum::{http::StatusCode, Json};
use diesel::prelude::*;

pub async fn list_users() -> Result<Json<Vec<ResUser>>, StatusCode> {
    use crate::db::schema::cities::dsl::*;
    use crate::db::schema::users::dsl::*;

    let conn = &mut establish_connection();

    let result = users
        .inner_join(cities.inner_join(countries::table))
        .select((User::as_select(), City::as_select(), Country::as_select()))
        .load::<(User, City, Country)>(conn)
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
        .collect();

    Ok(Json(result))
}
