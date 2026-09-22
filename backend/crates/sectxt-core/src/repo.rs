use crate::attachment::Attachment;
use crate::message::{Message, MessageMetadata, MessageWithAttachments};
use uuid::Uuid;

#[derive(Clone, Debug, thiserror::Error)]
pub enum MessageRepoError {
    #[error("{0}")]
    Database(String),

    #[error("{0}")]
    Model(String),
}

#[async_trait::async_trait]
pub trait MessageRepo: Send + Sync {
    async fn clean(&self) -> Result<u64, MessageRepoError>;
    async fn consume(&self, id: Uuid) -> Result<Option<MessageWithAttachments>, MessageRepoError>;
    async fn count_attachments(&self, message_id: Uuid) -> Result<i64, MessageRepoError>;
    async fn create_attachment(&self, attachment: Attachment) -> Result<Uuid, MessageRepoError>;
    async fn create_message(&self, message: Message) -> Result<Uuid, MessageRepoError>;
    async fn get_metadata(&self, id: Uuid) -> Result<MessageMetadata, MessageRepoError>;
}
