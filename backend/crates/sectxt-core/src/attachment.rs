use uuid::Uuid;

#[derive(Clone, Debug, thiserror::Error)]
pub enum AttachmentError {
    #[error("ciphertext cannot be empty")]
    EmptyCiphertext,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Attachment {
    id: Uuid,
    message_id: Uuid,
    ciphertext: Vec<u8>,
    nonce: [u8; 12],
    salt: [u8; 16],
}

impl Attachment {
    #[inline]
    pub fn builder() -> RawAttachmentBuilder {
        RawAttachment::builder()
    }

    #[inline]
    #[must_use]
    pub const fn id(&self) -> Uuid {
        self.id
    }

    #[inline]
    #[must_use]
    pub const fn message_id(&self) -> Uuid {
        self.message_id
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

#[derive(bon::Builder)]
#[builder(finish_fn = build_raw)]
pub struct RawAttachment {
    id: Option<Uuid>,
    message_id: Uuid,
    #[builder(into)]
    ciphertext: Vec<u8>,
    nonce: [u8; 12],
    salt: [u8; 16],
}

impl<State: raw_attachment_builder::IsComplete> RawAttachmentBuilder<State> {
    pub fn build(self) -> Result<Attachment, AttachmentError> {
        let raw = self.build_raw();
        if raw.ciphertext.is_empty() {
            return Err(AttachmentError::EmptyCiphertext);
        }

        Ok(Attachment {
            id: raw.id.unwrap_or_else(Uuid::now_v7),
            message_id: raw.message_id,
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

    fn create_attachment(message_id: Uuid) -> Attachment {
        Attachment::builder()
            .message_id(message_id)
            .ciphertext(vec![0; 128])
            .nonce([0; 12])
            .salt([0; 16])
            .build()
            .unwrap()
    }

    #[test]
    fn test_builder() {
        let message_id = Uuid::now_v7();
        let sut = create_attachment(message_id);
        assert!(!sut.id.is_nil());
        assert_eq!(sut.message_id, message_id);
        assert_eq!(sut.ciphertext, vec![0; 128]);
        assert_eq!(sut.nonce, [0; 12]);
        assert_eq!(sut.salt, [0; 16]);
    }

    #[test]
    fn test_empty_ciphertext() {
        let sut = Attachment::builder()
            .message_id(Uuid::now_v7())
            .ciphertext(vec![])
            .nonce([0; 12])
            .salt([0; 16])
            .build();

        assert_matches!(sut, Err(AttachmentError::EmptyCiphertext));
    }
}
