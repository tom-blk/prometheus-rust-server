#![allow(unused)]

pub use self::error::{Error, Result};

use std::{fmt::format, net::SocketAddr};

use axum::{extract::{Path, Query}, middleware, response::{Html, IntoResponse, Response}, routing::{get, get_service}, Router};
use model::ModelController;
use serde::Deserialize;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use web::mw_auth::mw_require_auth;

mod error;
mod model;
mod web;
mod ctx;

#[tokio::main]
async fn main() -> Result<()> {

    // region ----- Init Model Contoller
    let mc = ModelController::new().await?;

    let routes_api = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(mw_require_auth));

    let routes_all = Router::new()
        .merge(web::routes_login::routes())
        .merge(routes_all()) //merge all the routes under the route the current router operates on
        .nest("/api", routes_api) //same as above, but nest the routes under /api
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    // region ----- Start Server

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    println!("Listening on port {:?}\n", listener.local_addr());

    axum::serve(listener, routes_all).await.unwrap();

    // endregion ----- Start Server

    Ok(())
}

async fn main_response_mapper(res: Response) -> Response{
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    println!();
    res
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

