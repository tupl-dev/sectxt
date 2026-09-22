use sectxt_core::repo::MessageRepo;

#[derive(Clone, Debug)]
pub struct AppEnvironment {
    pub api_address: String,
    pub database_url: String,
}

impl AppEnvironment {
    #[inline]
    pub fn from_env() -> Result<Self, std::env::VarError> {
        Ok(Self {
            api_address: std::env::var("API_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_owned()),
            database_url: std::env::var("DATABASE_URL")?,
        })
    }
}

pub struct AppState {
    pub message_repo: Box<dyn MessageRepo>,
}

impl AppState {
    #[inline]
    #[must_use]
    pub const fn new(message_repo: Box<dyn MessageRepo>) -> Self {
        Self { message_repo }
    }
}
