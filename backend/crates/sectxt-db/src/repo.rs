use crate::attachment::AttachmentModel;
use crate::message::{MessageMetadataModel, MessageModel};
use sectxt_core::attachment::Attachment;
use sectxt_core::message::{Message, MessageMetadata, MessageWithAttachments};
use sectxt_core::repo::{MessageRepo, MessageRepoError};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct PgMessageRepo {
    pool: PgPool,
}

impl PgMessageRepo {
    #[inline]
    #[must_use]
    pub const fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn get_message<'a, E>(&self, id: Uuid, exec: E) -> Result<Option<Message>, MessageRepoError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let sql = r#"
            SELECT "id", "created_at", "expires_at", "burn_on_read", "ciphertext", "nonce", "salt"
            FROM "public"."active_messages" WHERE "id" = $1
            FOR UPDATE;"#;

        sqlx::query_as::<_, MessageModel>(sql)
            .bind(id)
            .fetch_optional(exec)
            .await
            .map_err(|e| MessageRepoError::Database(e.to_string()))?
            .map(Message::try_from)
            .transpose()
            .map_err(|e| MessageRepoError::Database(e.to_string()))
    }

    async fn get_attachments<'a, E>(&self, message_id: Uuid, exec: E) -> Result<Vec<Attachment>, MessageRepoError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let sql = r#"
            SELECT "id", "message_id", "ciphertext", "nonce", "salt"
            FROM "public"."attachments" WHERE "message_id" = $1
            FOR UPDATE;"#;

        sqlx::query_as::<_, AttachmentModel>(sql)
            .bind(message_id)
            .fetch_all(exec)
            .await
            .map_err(|e| MessageRepoError::Database(e.to_string()))
            .map(|items| {
                items
                    .into_iter()
                    .filter_map(|model| Attachment::try_from(model).ok())
                    .collect()
            })
    }

    async fn delete_message<'a, E>(&self, id: Uuid, exec: E) -> Result<(), MessageRepoError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        sqlx::query(r#"DELETE FROM "public"."messages" WHERE "id" = $1;"#)
            .bind(id)
            .execute(exec)
            .await
            .map(|_| ())
            .map_err(|e| MessageRepoError::Database(e.to_string()))
    }
}

#[async_trait::async_trait]
impl MessageRepo for PgMessageRepo {
    async fn clean(&self) -> Result<u64, MessageRepoError> {
        sqlx::query(r#"DELETE FROM "public"."messages" WHERE "expires_at" <= NOW();"#)
            .execute(&self.pool)
            .await
            .map(|r| r.rows_affected())
            .map_err(|e| MessageRepoError::Database(e.to_string()))
    }

    async fn consume(&self, id: Uuid) -> Result<Option<MessageWithAttachments>, MessageRepoError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| MessageRepoError::Database(e.to_string()))?;

        let Some(message) = self.get_message(id, &mut *tx).await? else {
            return Ok(None);
        };
        let attachments = self.get_attachments(id, &mut *tx).await?;
        if message.burn_on_read() {
            self.delete_message(id, &mut *tx).await?;
        }

        tx.commit().await.map_err(|e| MessageRepoError::Database(e.to_string()))?;
        Ok(Some(MessageWithAttachments { message, attachments }))
    }

    async fn count_attachments(&self, message_id: Uuid) -> Result<i64, MessageRepoError> {
        sqlx::query_scalar::<_, i64>(r#"SELECT COUNT(*) FROM "public"."attachments" WHERE "message_id" = $1;"#)
            .bind(message_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| MessageRepoError::Database(e.to_string()))
    }

    async fn create_attachment(&self, attachment: Attachment) -> Result<Uuid, MessageRepoError> {
        let sql = r#"
            INSERT INTO "public"."attachments" ("id", "message_id", "ciphertext", "nonce", "salt")
            VALUES ($1, $2, $3, $4, $5) RETURNING "id";"#;

        sqlx::query_scalar::<_, Uuid>(sql)
            .bind(attachment.id())
            .bind(attachment.message_id())
            .bind(attachment.ciphertext())
            .bind(attachment.nonce())
            .bind(attachment.salt())
            .fetch_one(&self.pool)
            .await
            .map_err(|e| MessageRepoError::Database(e.to_string()))
    }

    async fn create_message(&self, message: Message) -> Result<Uuid, MessageRepoError> {
        let sql = r#"
            INSERT INTO "public"."messages" ("id", "created_at", "expires_at", "burn_on_read", "ciphertext", "nonce", "salt")
            VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING "id";"#;

        sqlx::query_scalar::<_, Uuid>(sql)
            .bind(message.id())
            .bind(message.created_at())
            .bind(message.expires_at())
            .bind(message.burn_on_read())
            .bind(message.ciphertext())
            .bind(message.nonce())
            .bind(message.salt())
            .fetch_one(&self.pool)
            .await
            .map_err(|e| MessageRepoError::Database(e.to_string()))
    }

    async fn get_metadata(&self, id: Uuid) -> Result<MessageMetadata, MessageRepoError> {
        let sql = r#"SELECT "burn_on_read" FROM "public"."active_messages" WHERE "id" = $1;"#;
        sqlx::query_as::<_, MessageMetadataModel>(sql)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map(|o| o.unwrap_or_default().into())
            .map_err(|e| MessageRepoError::Database(e.to_string()))
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};
    use std::assert_matches;

    fn create_burn_message() -> Message {
        Message::builder()
            .burn_on_read(true)
            .ciphertext(vec![0; 32])
            .nonce([0; 12])
            .salt([0; 16])
            .build()
            .unwrap()
    }

    fn create_expired_message() -> Message {
        Message::builder()
            .created_at(Utc::now() - Duration::days(30))
            .ciphertext(vec![0; 32])
            .nonce([0; 12])
            .salt([0; 16])
            .build()
            .unwrap()
    }

    fn create_message() -> Message {
        Message::builder()
            .ciphertext(vec![0; 32])
            .nonce([0; 12])
            .salt([0; 16])
            .build()
            .unwrap()
    }

    fn create_attachment(message_id: Uuid) -> Attachment {
        Attachment::builder()
            .message_id(message_id)
            .ciphertext(vec![0; 128])
            .nonce([0; 12])
            .salt([0; 16])
            .build()
            .unwrap()
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_clean_messages_expired(pool: PgPool) {
        let sut = PgMessageRepo::new(pool);
        let _ = sut.create_message(create_expired_message()).await.unwrap();
        let result = sut.clean().await.unwrap();
        assert_eq!(result, 1);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_clean_messages_unaffected(pool: PgPool) {
        let sut = PgMessageRepo::new(pool);
        let _ = sut.create_message(create_message()).await.unwrap();
        let result = sut.clean().await.unwrap();
        assert_eq!(result, 0);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_consume(pool: PgPool) {
        let message = Message::builder()
            .ciphertext(vec![0; 32])
            .nonce([0; 12])
            .salt([0; 16])
            .build()
            .unwrap();

        let sut = PgMessageRepo::new(pool);
        let id = sut.create_message(message.clone()).await.unwrap();
        let result = sut.consume(id).await.unwrap().unwrap();
        assert_eq!(result.message.ciphertext(), message.ciphertext());
        assert_eq!(result.message.nonce(), message.nonce());
        assert_eq!(result.message.salt(), message.salt());

        let result = sut.consume(id).await.unwrap();
        assert_matches!(result, Some(_));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_consume_attachments(pool: PgPool) {
        let message_id = Uuid::now_v7();
        let message = Message::builder()
            .id(message_id)
            .burn_on_read(true)
            .ciphertext(vec![0; 32])
            .nonce([0; 12])
            .salt([0; 16])
            .build()
            .unwrap();

        let attachment1 = create_attachment(message_id);
        let attachment2 = create_attachment(message_id);
        let sut = PgMessageRepo::new(pool);
        sut.create_message(message.clone()).await.unwrap();
        sut.create_attachment(attachment1.clone()).await.unwrap();
        sut.create_attachment(attachment2.clone()).await.unwrap();
        let result = sut.consume(message_id).await.unwrap().unwrap();
        assert_eq!(result.message.ciphertext(), message.ciphertext());
        assert_eq!(result.message.nonce(), message.nonce());
        assert_eq!(result.message.salt(), message.salt());
        assert_eq!(result.attachments.len(), 2);

        let result = sut.consume(message_id).await.unwrap();
        assert_eq!(result, None);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_consume_burnt(pool: PgPool) {
        let sut = PgMessageRepo::new(pool);
        let id = sut.create_message(create_burn_message()).await.unwrap();
        let result = sut.consume(id).await.unwrap();
        assert!(result.is_some());

        let result = sut.consume(id).await.unwrap();
        assert!(result.is_none());
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_consume_expired(pool: PgPool) {
        let sut = PgMessageRepo::new(pool);
        let id = sut.create_message(create_expired_message()).await.unwrap();
        let result = sut.consume(id).await.unwrap();
        assert_eq!(result, None);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_count_attachments(pool: PgPool) {
        let sut = PgMessageRepo::new(pool);
        let message = create_message();
        sut.create_message(message.clone()).await.unwrap();
        sut.create_attachment(create_attachment(message.id())).await.unwrap();
        sut.create_attachment(create_attachment(message.id())).await.unwrap();
        sut.create_attachment(create_attachment(message.id())).await.unwrap();
        let result = sut.count_attachments(message.id()).await.unwrap();
        assert_eq!(result, 3);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_create_attachment(pool: PgPool) {
        let message = create_message();
        let attachment = create_attachment(message.id());
        let sut = PgMessageRepo::new(pool);
        sut.create_message(message).await.unwrap();
        let result = sut.create_attachment(attachment.clone()).await.unwrap();
        assert_eq!(result, attachment.id());
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_create_attachment_message_not_found(pool: PgPool) {
        let sut = PgMessageRepo::new(pool);
        let attachment = create_attachment(Uuid::now_v7());
        let result = sut.create_attachment(attachment).await;
        assert_matches!(result, Err(MessageRepoError::Database(_)));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_create_message(pool: PgPool) {
        let message = create_message();
        let sut = PgMessageRepo::new(pool);
        let result = sut.create_message(message.clone()).await.unwrap();
        assert_eq!(result, message.id());
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_metadata(pool: PgPool) {
        let sut = PgMessageRepo::new(pool);
        let id = sut.create_message(create_burn_message()).await.unwrap();
        let result = sut.get_metadata(id).await.unwrap();
        assert!(result.burn_on_read);

        let id = sut.create_message(create_message()).await.unwrap();
        let result = sut.get_metadata(id).await.unwrap();
        assert!(!result.burn_on_read);
    }
}
