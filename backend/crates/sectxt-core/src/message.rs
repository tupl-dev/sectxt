use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, thiserror::Error)]
pub enum MessageError {
    #[error("expires_at must be greater than created_at")]
    ExpiresAt,

    #[error("ciphertext must not be empty")]
    EmptyCiphertext,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    id: Uuid,
    created_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
    burn_on_read: bool,
    ciphertext: Vec<u8>,
    nonce: [u8; 12],
    salt: [u8; 16],
}

impl Message {
    #[inline]
    pub fn builder() -> RawMessageBuilder {
        RawMessage::builder()
    }

    #[inline]
    #[must_use]
    pub const fn id(&self) -> Uuid {
        self.id
    }

    #[inline]
    #[must_use]
    pub const fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    #[inline]
    #[must_use]
    pub const fn expires_at(&self) -> DateTime<Utc> {
        self.expires_at
    }

    #[inline]
    #[must_use]
    pub const fn burn_on_read(&self) -> bool {
        self.burn_on_read
    }

    #[inline]
    #[must_use]
    pub fn ciphertext(&self) -> &[u8] {
        &self.ciphertext
    }

    #[inline]
    #[must_use]
    pub const fn nonce(&self) -> [u8; 12] {
        self.nonce
    }

    #[inline]
    #[must_use]
    pub const fn salt(&self) -> [u8; 16] {
        self.salt
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MessageWithAttachments {
    pub message: Message,
    pub attachments: Vec<crate::attachment::Attachment>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MessageMetadata {
    pub burn_on_read: bool,
}

#[derive(bon::Builder)]
#[builder(finish_fn = build_raw)]
pub struct RawMessage {
    id: Option<Uuid>,
    created_at: Option<DateTime<Utc>>,
    expires_at: Option<DateTime<Utc>>,
    burn_on_read: Option<bool>,
    #[builder(into)]
    ciphertext: Vec<u8>,
    nonce: [u8; 12],
    salt: [u8; 16],
}

impl<State: raw_message_builder::IsComplete> RawMessageBuilder<State> {
    pub fn build(self) -> Result<Message, MessageError> {
        let raw = self.build_raw();
        let id = raw.id.unwrap_or_else(Uuid::now_v7);
        let created_at = raw.created_at.unwrap_or_else(Utc::now);
        let expires_at = raw.expires_at.unwrap_or_else(|| created_at + Duration::days(7));
        let burn_on_read = raw.burn_on_read.unwrap_or(false);
        if expires_at <= created_at {
            return Err(MessageError::ExpiresAt);
        }
        if raw.ciphertext.is_empty() {
            return Err(MessageError::EmptyCiphertext);
        }

        Ok(Message {
            id,
            created_at,
            expires_at,
            burn_on_read,
            ciphertext: raw.ciphertext,
            nonce: raw.nonce,
            salt: raw.salt,
        })
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use std::assert_matches;

    #[test]
    fn test_builder() {
        let time = Utc::now();
        let sut = Message::builder()
            .created_at(time)
            .expires_at(time + Duration::days(7))
            .burn_on_read(true)
            .ciphertext(vec![0; 128])
            .nonce([0; 12])
            .salt([0; 16])
            .build()
            .unwrap();

        assert_eq!(sut.created_at, time);
        assert_eq!(sut.expires_at, time + Duration::days(7));
        assert!(sut.burn_on_read);
        assert_eq!(sut.salt, [0; 16]);
        assert_eq!(sut.nonce, [0; 12]);
        assert_eq!(sut.ciphertext, vec![0; 128]);

        let time = Utc::now() - Duration::days(7);
        let sut = Message::builder()
            .created_at(time)
            .ciphertext(vec![0; 128])
            .nonce([0; 12])
            .salt([0; 16])
            .build()
            .unwrap();

        assert_eq!(sut.created_at, time);
        assert_eq!(sut.expires_at, time + Duration::days(7));
        assert!(!sut.burn_on_read);
        assert_eq!(sut.salt, [0; 16]);
        assert_eq!(sut.nonce, [0; 12]);
        assert_eq!(sut.ciphertext, vec![0; 128]);
    }

    #[test]
    fn test_invalid_expires_at() {
        let sut = Message::builder()
            .created_at(Utc::now())
            .expires_at(Utc::now() - Duration::days(1))
            .burn_on_read(true)
            .ciphertext(vec![0; 128])
            .nonce([0; 12])
            .salt([0; 16])
            .build();

        assert_matches!(sut, Err(MessageError::ExpiresAt));
    }

    #[test]
    fn test_empty_ciphertext() {
        let sut = Message::builder()
            .created_at(Utc::now())
            .expires_at(Utc::now() + Duration::days(7))
            .burn_on_read(true)
            .ciphertext(vec![])
            .nonce([0; 12])
            .salt([0; 16])
            .build();

        assert_matches!(sut, Err(MessageError::EmptyCiphertext));
    }
}
