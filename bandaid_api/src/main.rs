mod db;
mod routing;

use std::net::SocketAddr;

use dotenvy::dotenv;
use routing::create_routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let router = create_routes();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
