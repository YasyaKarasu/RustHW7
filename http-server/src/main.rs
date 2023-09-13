#![feature(impl_trait_in_assoc_type)]

use axum::{routing::*, Router};
mod volo_gen;
mod handler;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/ping", get(handler::ping))
        .route("/value", get(handler::get_value)
            .post(handler::set_value)
            .delete(handler::delete_value)
        );

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}