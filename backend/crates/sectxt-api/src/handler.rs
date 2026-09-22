use crate::dto::{
    CreateAttachmentRequest, CreateAttachmentResponse, CreateMessageRequest, CreateMessageResponse, GetMessageMetadataResponse,
    GetMessageWithAttachmentsResponse,
};
use crate::err::HandlerError;
use crate::state::AppState;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sectxt_core::message::Message;
use sqlx::types::Uuid;
use std::sync::Arc;

#[utoipa::path(
    post,
    path = "/message",
    tags = ["message"],
    request_body = CreateMessageRequest,
    responses(
        (status = 201, description = "Created", body = CreateMessageResponse),
        HandlerError
    )
)]
pub async fn create_message(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateMessageRequest>,
) -> Result<impl IntoResponse, HandlerError> {
    let message = Message::try_from(req).map_err(|e| HandlerError::BadRequest(e.to_string()))?;
    let id = state
        .message_repo
        .create_message(message)
        .await
        .map_err(|e| HandlerError::Server(e.to_string()))?;

    Ok((StatusCode::CREATED, Json(CreateMessageResponse { id })))
}

#[utoipa::path(
    post,
    path = "/message/{message_id}/attachment",
    tags = ["attachment"],
    request_body = CreateAttachmentRequest,
    params(
        ("message_id" = Uuid, Path, description = "The ID of the parent message")
    ),
    responses(
        (status = 201, description = "Created", body = CreateAttachmentResponse),
        HandlerError
    )
)]
pub async fn create_attachment(
    State(state): State<Arc<AppState>>,
    Path(message_id): Path<Uuid>,
    Json(req): Json<CreateAttachmentRequest>,
) -> Result<impl IntoResponse, HandlerError> {
    let count = state
        .message_repo
        .count_attachments(message_id)
        .await
        .map_err(|e| HandlerError::Server(e.to_string()))?;
    if count >= 10 {
        return Err(HandlerError::BadRequest("too many attachments".into()));
    }

    let attachment = req
        .into_core(message_id)
        .map_err(|e| HandlerError::BadRequest(e.to_string()))?;
    let id = state
        .message_repo
        .create_attachment(attachment)
        .await
        .map_err(|e| HandlerError::Server(e.to_string()))?;

    Ok((StatusCode::CREATED, Json(CreateAttachmentResponse { id })))
}

#[utoipa::path(
    get,
    path = "/message/{id}",
    tags = ["message"],
    params(
        ("id" = Uuid, Path, description = "The ID of the target message")
    ),
    responses(
        (status = 200, description = "Success", body = GetMessageWithAttachmentsResponse),
        HandlerError
    )
)]
pub async fn get_message(State(state): State<Arc<AppState>>, Path(id): Path<Uuid>) -> Result<impl IntoResponse, HandlerError> {
    let mwa = state
        .message_repo
        .consume(id)
        .await
        .map_err(|e| HandlerError::Server(e.to_string()))?
        .ok_or_else(|| HandlerError::NotFound("message not found".to_owned()))?;

    Ok((StatusCode::OK, Json(GetMessageWithAttachmentsResponse::from(mwa))))
}

#[utoipa::path(
    get,
    path = "/message/{id}/metadata",
    tags = ["message"],
    params(
        ("id" = Uuid, Path, description = "The ID of the target message")
    ),
    responses(
        (status = 200, description = "Success", body = GetMessageMetadataResponse),
        HandlerError
    )
)]
pub async fn get_metadata(State(state): State<Arc<AppState>>, Path(id): Path<Uuid>) -> Result<impl IntoResponse, HandlerError> {
    let metadata = state
        .message_repo
        .get_metadata(id)
        .await
        .map_err(|e| HandlerError::Server(e.to_string()))?;

    Ok((StatusCode::OK, Json(GetMessageMetadataResponse::from(metadata))))
}
