#![feature(impl_trait_in_assoc_type)]

use lazy_static::lazy_static;
use std::net::SocketAddr;
use axum::{routing::*, Router};
mod volo_gen;
mod handler;

lazy_static! {
    static ref CLIENT: crate::volo_gen::volo_gen::mini::redis::MiniRedisServiceClient = {
        let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        crate::volo_gen::volo_gen::mini::redis::MiniRedisServiceClientBuilder::new("mini-redis")
            .address(addr)
            .build()
    };
}

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