#![feature(once_cell)]

use std::{sync::OnceLock, time::Duration};

use axum::{Router, routing::get};
use reqwest::Client;

mod error;
mod routes;

static CLIENT: OnceLock<Client> = OnceLock::new();

pub fn get_client() -> &'static Client {
    CLIENT.get_or_init(|| {
        Client::builder()
            .timeout(Duration::from_secs(5))
            .build() // Set user agent later.
            .expect("Failed to build client")
    })
}

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/proxy", get(routes::proxy::proxy));

    axum::Server::bind(&"0.0.0.0:8079".parse().unwrap())
        .serve(router.into_make_service())
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c()
                .await
                .expect("Failed to listen to ctrl_c")
        })
        .await
        .expect("Failed to run server");
}
