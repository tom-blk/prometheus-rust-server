use axum::body::Body;
use axum::extract::{FromRequestParts, Request};
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::response::Response;
use axum::RequestPartsExt;
use lazy_regex::regex_captures;
use tower_cookies::Cookies;

use crate::ctx::Ctx;
use crate::web::AUTH_TOKEN;
use crate::{Error, Result};

pub async fn mw_require_auth(
    ctx: Result<Ctx>,
    req: Request<Body>, 
    next: Next
) -> Result<Response> {

    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");

    ctx?;

    Ok(next.run(req).await)
}

    
// region ----- Ctx Extractors

#[async_trait::async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("->> {:<12} - Ctx", "EXTRACTOR"); 

        // User cookie extraction
        
        let cookies = parts.extract::<Cookies>().await.unwrap();

        let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

        // Parse token
        let (user_id, exp, sign) = auth_token
            .ok_or(Error::AuthFailNoAuthTokenCookie)
            .and_then(parse_token)?;

        // TODO Token validation
        
        Ok(Ctx::new(user_id))
    }
    
}

// endregion ----- Ctx Extractors


// Parse token of format 'user-[user-id].[expiration-time].[signature]'
// Returns user_id, exipration, signature
fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, expiration, signature) = regex_captures!(
        r#"^user-(\d+)\.(.+)\.(.+)"#,
		&token
    )
    .ok_or(Error::AuthFailTokenWrongFormat)?;
    
    let user_id: u64 = user_id
        .parse()
        .map_err(|_| Error::AuthFailTokenWrongFormat)?;

    Ok((user_id, expiration.to_string(), signature.to_string()))
}
