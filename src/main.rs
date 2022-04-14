use std::net::SocketAddr;

use axum::Router;

#[tokio::main]
async fn main() {
    let app = Router::new();

    axum::Server::bind(&SocketAddr::from(([127, 0, 0, 1], 8080)))
        .serve(app.into_make_service())
        .await
        .unwrap();
}
