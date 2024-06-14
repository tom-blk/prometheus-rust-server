use axum::{http::StatusCode, response::{IntoResponse, Response}};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFail,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response{
        println!("--> {:<12} - {self:?}", "INTO_RES");

        //Don't pass server Error through to the client to avoid attack vector
        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDELED_CLIENT_ERROR").into_response()
    }
}

impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}
