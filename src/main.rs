#![allow(unused)]

use std::{fmt::format, net::SocketAddr};

use axum::{extract::{Path, Query}, response::{Html, IntoResponse}, routing::{get, get_service}, Router};
use serde::Deserialize;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let routes_all = Router::new().merge(routes_all());

    // region: --- Start Server

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    println!("Listening on port {:?}\n", listener.local_addr());

    axum::serve(listener, routes_all).await.unwrap();

}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

fn routes_all() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name_param", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams{
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");


    let name = params.name.as_deref().unwrap_or("World");

    Html(format!("Hello <strong>{name}</strong>"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2 - {name:?}", "HANDLER");

    Html(format!("Hello <strong>{name}</strong>"))
}

