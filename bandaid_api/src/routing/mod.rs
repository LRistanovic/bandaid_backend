mod mw;

mod cities;
mod countries;
mod users;

use axum::{
    http::Method,
    middleware,
    routing::{delete, get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

use self::{cities::*, countries::*, users::*};
use mw::guard::guard;

pub fn create_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_origin(Any);

    Router::new()
        .route("/users", delete(delete_user))
        .route_layer(middleware::from_fn(guard))
        .route("/users", get(list_users))
        .route("/users/register", post(create_user))
        .route("/users/login", post(login_user))
        .route("/countries", get(list_countries))
        .route("/cities", get(list_cities))
        .layer(cors)
}
