mod dto;
mod err;
mod handler;
mod state;
mod worker;

use crate::state::{AppEnvironment, AppState};
use axum::Router;
use axum::extract::DefaultBodyLimit;
use axum::http::{Method, header};
use sectxt_db::repo::PgMessageRepo;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tower_http::compression::CompressionLayer;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, HttpMakeClassifier, TraceLayer};
use tracing::Level;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

#[derive(OpenApi)]
#[openapi(servers((url = "/api")))]
struct ApiDoc;

fn get_router(state: Arc<AppState>) -> (Router, utoipa::openapi::OpenApi) {
    let (routes, openapi) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(handler::get_message))
        .routes(routes!(handler::get_metadata))
        .routes(routes!(handler::create_message))
        .routes(routes!(handler::create_attachment))
        .split_for_parts();

    let app = Router::new()
        .nest("/api", routes)
        .layer(CompressionLayer::new())
        .layer(get_trace_layer())
        .layer(get_cors_layer())
        .layer(DefaultBodyLimit::max(20 * 1024 * 1024))
        .with_state(state);

    (app, openapi)
}

async fn get_pg_pool(db_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(20)
        .idle_timeout(std::time::Duration::from_secs(30))
        .connect(db_url)
        .await
}

fn get_trace_layer() -> TraceLayer<HttpMakeClassifier> {
    TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO))
}

fn get_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE, header::ACCEPT])
}

#[cfg(debug_assertions)]
fn merge_swagger(app: Router, openapi: utoipa::openapi::OpenApi) -> Router {
    app.merge(utoipa_swagger_ui::SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", openapi))
}

#[tokio::main]
#[allow(clippy::unwrap_used)]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();
    let env = AppEnvironment::from_env().unwrap();
    let db_pool = get_pg_pool(&env.database_url).await.unwrap();
    let message_repo = PgMessageRepo::new(db_pool);
    let state = Arc::new(AppState::new(Box::new(message_repo)));
    let listener = tokio::net::TcpListener::bind(&env.api_address).await.unwrap();

    #[cfg(debug_assertions)]
    let (mut app, openapi) = get_router(state.clone());
    #[cfg(not(debug_assertions))]
    let (app, _) = get_router(state.clone());

    #[cfg(debug_assertions)]
    {
        app = merge_swagger(app, openapi);
        tracing::info!("Swagger UI active at {}/swagger-ui", &env.api_address);
    }

    worker::clean_expired_messages(state.clone(), 10);
    tracing::info!("Listening on {}", &env.api_address);
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::dto::{
        CreateAttachmentRequest, CreateAttachmentResponse, CreateMessageRequest, CreateMessageResponse,
        GetMessageMetadataResponse, GetMessageWithAttachmentsResponse,
    };
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use http_body_util::BodyExt;
    use sqlx::types::Uuid;
    use std::fs::File;
    use std::io::Write;
    use tower::ServiceExt;

    fn get_app(pool: PgPool) -> (Router, utoipa::openapi::OpenApi) {
        let message_repo = PgMessageRepo::new(pool);
        let state = Arc::new(AppState::new(Box::new(message_repo)));
        get_router(state)
    }

    fn get_create_message_json(burn_on_read: bool) -> String {
        let req = CreateMessageRequest {
            burn_on_read,
            ciphertext: vec![0; 128],
            nonce: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            salt: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            ttl_seconds: None,
        };
        serde_json::to_string(&req).unwrap()
    }

    fn get_create_attachment_json() -> String {
        let req = CreateAttachmentRequest {
            ciphertext: vec![0; 128],
            nonce: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            salt: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        };
        serde_json::to_string(&req).unwrap()
    }

    async fn create_message(app: Router, burn_on_read: bool) -> Uuid {
        let request = Request::builder()
            .method("POST")
            .uri("/api/message")
            .header("content-type", "application/json")
            .body(Body::from(get_create_message_json(burn_on_read)))
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        serde_json::from_slice::<CreateMessageResponse>(&bytes).unwrap().id
    }

    #[sqlx::test(migrations = "../sectxt-db/migrations")]
    async fn generate_openapi_doc(pool: PgPool) {
        let (_, openapi) = get_app(pool);
        let json = openapi.to_pretty_json().unwrap();
        let mut file = File::create("../../openapi.json").unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }

    #[sqlx::test(migrations = "../sectxt-db/migrations")]
    async fn test_create_message(pool: PgPool) {
        let (sut, _) = get_app(pool);
        let request = Request::builder()
            .method("POST")
            .uri("/api/message")
            .header("content-type", "application/json")
            .body(Body::from(get_create_message_json(false)))
            .unwrap();

        let response = sut.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        let resp: CreateMessageResponse = serde_json::from_slice(&bytes).unwrap();
        assert!(!resp.id.is_nil());
    }

    #[sqlx::test(migrations = "../sectxt-db/migrations")]
    async fn test_create_message_bad_json(pool: PgPool) {
        let (sut, _) = get_app(pool);
        let json = r#"{"ciphertext": [0]}"#;
        let request = Request::builder()
            .method("POST")
            .uri("/api/message")
            .header("content-type", "application/json")
            .body(Body::from(json))
            .unwrap();

        let response = sut.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[sqlx::test(migrations = "../sectxt-db/migrations")]
    async fn test_create_attachment(pool: PgPool) {
        let (sut, _) = get_app(pool);
        let message_id = create_message(sut.clone(), false).await;
        let request = Request::builder()
            .method("POST")
            .uri(format!("/api/message/{message_id}/attachment"))
            .header("content-type", "application/json")
            .body(Body::from(get_create_attachment_json()))
            .unwrap();

        let response = sut.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        let resp: CreateAttachmentResponse = serde_json::from_slice(&bytes).unwrap();
        assert!(!resp.id.is_nil());
    }

    #[sqlx::test(migrations = "../sectxt-db/migrations")]
    async fn test_create_attachment_with_no_parent(pool: PgPool) {
        let (sut, _) = get_app(pool);
        let request = Request::builder()
            .method("POST")
            .uri(format!("/api/message/{}/attachment", Uuid::now_v7()))
            .header("content-type", "application/json")
            .body(Body::from(get_create_attachment_json()))
            .unwrap();

        let response = sut.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[sqlx::test(migrations = "../sectxt-db/migrations")]
    async fn test_get_message(pool: PgPool) {
        let (sut, _) = get_app(pool);
        let id = create_message(sut.clone(), false).await;
        let request = Request::builder()
            .method("GET")
            .uri(format!("/api/message/{id}"))
            .header("content-type", "application/json")
            .body(Body::empty())
            .unwrap();

        let response = sut.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        let dto: GetMessageWithAttachmentsResponse = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(dto.message.ciphertext, vec![0; 128]);
        assert_eq!(dto.message.nonce, [0; 12]);
        assert_eq!(dto.message.salt, [0; 16]);
        assert_eq!(dto.attachments.len(), 0);
    }

    #[sqlx::test(migrations = "../sectxt-db/migrations")]
    async fn test_get_burnt_message(pool: PgPool) {
        let (sut, _) = get_app(pool);
        let id = create_message(sut.clone(), true).await;
        let make_request = |id| {
            Request::builder()
                .method("GET")
                .uri(format!("/api/message/{id}"))
                .header("content-type", "application/json")
                .body(Body::empty())
                .unwrap()
        };

        let response = sut.clone().oneshot(make_request(id)).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let response = sut.oneshot(make_request(id)).await.unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[sqlx::test(migrations = "../sectxt-db/migrations")]
    async fn test_get_metadata(pool: PgPool) {
        let (sut, _) = get_app(pool);
        let make_request = |id| {
            Request::builder()
                .method("GET")
                .uri(format!("/api/message/{id}/metadata"))
                .header("content-type", "application/json")
                .body(Body::empty())
                .unwrap()
        };

        let id = create_message(sut.clone(), true).await;
        let response = sut.clone().oneshot(make_request(id)).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        let metadata: GetMessageMetadataResponse = serde_json::from_slice(&bytes).unwrap();
        assert!(metadata.burn_on_read);

        let id = create_message(sut.clone(), false).await;
        let response = sut.clone().oneshot(make_request(id)).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        let metadata: GetMessageMetadataResponse = serde_json::from_slice(&bytes).unwrap();
        assert!(!metadata.burn_on_read);
    }
}
