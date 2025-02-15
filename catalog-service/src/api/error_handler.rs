use axum::{response::{ IntoResponse, Response},  http::StatusCode};

pub struct AppError(anyhow::Error);

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        //TODO: customize the response.
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Something went wrong: {}", self.0)).into_response()
    }
}