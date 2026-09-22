use sectxt_core::attachment::{Attachment, AttachmentError};
use uuid::Uuid;

#[derive(Clone, Debug, thiserror::Error)]
pub enum AttachmentModelError {
    #[error("{0}")]
    Domain(#[from] AttachmentError),
}

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow)]
pub struct AttachmentModel {
    pub id: Uuid,
    pub message_id: Uuid,
    pub ciphertext: Vec<u8>,
    pub nonce: [u8; 12],
    pub salt: [u8; 16],
}

impl From<Attachment> for AttachmentModel {
    fn from(value: Attachment) -> Self {
        Self {
            id: value.id(),
            message_id: value.message_id(),
            ciphertext: value.ciphertext().to_vec(),
            nonce: value.nonce(),
            salt: value.salt(),
        }
    }
}

impl TryFrom<AttachmentModel> for Attachment {
    type Error = AttachmentModelError;

    fn try_from(value: AttachmentModel) -> Result<Self, Self::Error> {
        Ok(Self::builder()
            .id(value.id)
            .message_id(value.message_id)
            .ciphertext(value.ciphertext)
            .nonce(value.nonce)
            .salt(value.salt)
            .build()?)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use crate::attachment::AttachmentModel;
    use sectxt_core::attachment::Attachment;
    use uuid::Uuid;

    #[test]
    fn test_from_attachment() {
        let attachment = Attachment::builder()
            .id(Uuid::now_v7())
            .message_id(Uuid::now_v7())
            .ciphertext(vec![0; 128])
            .nonce([0; 12])
            .salt([0; 16])
            .build()
            .unwrap();

        let sut = AttachmentModel::from(attachment.clone());
        assert_eq!(sut.id, attachment.id());
        assert_eq!(sut.message_id, attachment.message_id());
        assert_eq!(sut.ciphertext, attachment.ciphertext());
        assert_eq!(sut.nonce, attachment.nonce());
        assert_eq!(sut.salt, attachment.salt());
    }

    #[test]
    fn test_from_attachment_model() {
        let sut = AttachmentModel {
            id: Uuid::now_v7(),
            message_id: Uuid::now_v7(),
            ciphertext: vec![0; 128],
            nonce: [0; 12],
            salt: [0; 16],
        };

        let attachment = Attachment::try_from(sut.clone()).unwrap();
        assert_eq!(attachment.id(), sut.id);
        assert_eq!(attachment.message_id(), sut.message_id);
        assert_eq!(attachment.ciphertext(), sut.ciphertext);
        assert_eq!(attachment.nonce(), sut.nonce);
        assert_eq!(attachment.salt(), sut.salt);
    }
}
