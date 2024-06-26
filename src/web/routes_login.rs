use crate::{web, Error, Result};
use serde::Deserialize;
use serde_json::{json, Value};
use axum::{routing::post, Json, Router};
use tower_cookies::{Cookies, Cookie};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    //TODO implement database authentication
    
    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(Error::LoginFail);
    }

    //TODO Implement real auth token
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));
    
    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}
