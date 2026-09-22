use chrono::{DateTime, Utc};
use sectxt_core::message::{Message, MessageError, MessageMetadata};
use uuid::Uuid;

#[derive(Clone, Debug, thiserror::Error)]
pub enum MessageModelError {
    #[error("{0}")]
    Domain(#[from] MessageError),
}

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow)]
pub struct MessageModel {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub burn_on_read: bool,
    pub ciphertext: Vec<u8>,
    pub nonce: [u8; 12],
    pub salt: [u8; 16],
}

impl From<Message> for MessageModel {
    fn from(message: Message) -> Self {
        Self {
            id: message.id(),
            created_at: message.created_at(),
            expires_at: message.expires_at(),
            burn_on_read: message.burn_on_read(),
            ciphertext: message.ciphertext().to_vec(),
            nonce: message.nonce(),
            salt: message.salt(),
        }
    }
}

impl TryFrom<MessageModel> for Message {
    type Error = MessageModelError;

    fn try_from(value: MessageModel) -> Result<Self, Self::Error> {
        Ok(Self::builder()
            .id(value.id)
            .created_at(value.created_at)
            .expires_at(value.expires_at)
            .burn_on_read(value.burn_on_read)
            .ciphertext(value.ciphertext)
            .nonce(value.nonce)
            .salt(value.salt)
            .build()?)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, sqlx::FromRow)]
pub struct MessageMetadataModel {
    pub burn_on_read: bool,
}

impl From<MessageMetadataModel> for MessageMetadata {
    #[inline]
    fn from(value: MessageMetadataModel) -> Self {
        Self {
            burn_on_read: value.burn_on_read,
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::message::MessageModel;
    use sectxt_core::message::Message;
    use std::assert_matches;

    #[test]
    fn test_from() {
        let id = Uuid::now_v7();
        let message = Message::builder()
            .id(id)
            .burn_on_read(true)
            .ciphertext(vec![0; 128])
            .nonce([0; 12])
            .salt([0; 16])
            .build()
            .unwrap();

        let sut = MessageModel::from(message.clone());
        assert_eq!(sut.id, message.id());
        assert_eq!(sut.created_at, message.created_at());
        assert_eq!(sut.expires_at, message.expires_at());
        assert_eq!(sut.burn_on_read, message.burn_on_read());
        assert_eq!(sut.ciphertext, message.ciphertext());
        assert_eq!(sut.nonce, message.nonce());
        assert_eq!(sut.salt, message.salt());
    }

    #[test]
    fn test_try_from() {
        let now = Utc::now();
        let sut = MessageModel {
            id: Uuid::now_v7(),
            created_at: now,
            expires_at: now + chrono::Duration::days(7),
            burn_on_read: true,
            ciphertext: vec![0; 128],
            nonce: [0; 12],
            salt: [0; 16],
        };

        let message = Message::try_from(sut.clone()).unwrap();
        assert_eq!(message.id(), sut.id);
        assert_eq!(message.created_at(), sut.created_at);
        assert_eq!(message.expires_at(), sut.expires_at);
        assert_eq!(message.burn_on_read(), sut.burn_on_read);
        assert_eq!(message.ciphertext(), sut.ciphertext);
        assert_eq!(message.nonce(), sut.nonce);
        assert_eq!(message.salt(), sut.salt);
    }

    #[test]
    fn test_try_from_invalid_expires_at() {
        let sut = MessageModel {
            id: Uuid::now_v7(),
            created_at: Utc::now(),
            expires_at: Utc::now() - chrono::Duration::days(1),
            burn_on_read: true,
            ciphertext: vec![0; 128],
            nonce: [0; 12],
            salt: [0; 16],
        };

        let result = Message::try_from(sut);
        assert_matches!(result, Err(MessageModelError::Domain(MessageError::ExpiresAt)));
    }
}
