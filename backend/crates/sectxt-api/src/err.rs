use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Clone, Debug, thiserror::Error, utoipa::IntoResponses)]
pub enum HandlerError {
    #[error("{0}")]
    #[response(status = BAD_REQUEST, description = "Bad Request")]
    BadRequest(String),

    #[error("{0}")]
    #[response(status = NOT_FOUND, description = "Not Found")]
    NotFound(String),

    #[error("{0}")]
    #[response(status = INTERNAL_SERVER_ERROR, description = "Internal Server Error")]
    Server(String),
}

impl IntoResponse for HandlerError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            Self::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            Self::Server(msg) => {
                tracing::error!(error = %msg, "server error");
                (StatusCode::INTERNAL_SERVER_ERROR, "internal server error".to_owned())
            },
        };
        (status, message).into_response()
    }
}
