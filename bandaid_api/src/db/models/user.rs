use crate::db::schema::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::*;

#[derive(Queryable, Selectable, Associations, Clone)]
#[diesel(belongs_to(City))]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub description: String,
    pub birth_year: i32,
    pub city_id: i32,
    pub profile_picture_url: Option<String>,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub description: String,
    pub birth_year: i32,
    pub city_id: i32,
}

#[derive(Deserialize)]
pub struct LoginCredentials {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct Token {
    pub token: String,
}

#[derive(Serialize)]
pub struct ResUser {
    pub id: i32,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub description: String,
    pub birth_year: i32,
    pub city: ResCity,
    pub profile_picture_url: Option<String>,
}
