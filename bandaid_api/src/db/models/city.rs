use crate::db::schema::*;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Identifiable, Serialize)]
#[diesel(table_name = countries)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Country {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Selectable, Identifiable, Serialize, Associations)]
#[diesel(belongs_to(Country))]
#[diesel(table_name = cities)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct City {
    pub id: i32,
    pub name: String,
    pub country_id: i32,
}

#[derive(Serialize)]
pub struct ResCity {
    pub id: i32,
    pub name: String,
    pub country: Country,
}
