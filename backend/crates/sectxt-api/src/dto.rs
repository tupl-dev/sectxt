use sectxt_core::attachment::{Attachment, AttachmentError};
use sectxt_core::message::{Message, MessageError, MessageMetadata, MessageWithAttachments};
use serde::{Deserialize, Serialize};
use serde_with::{base64::Base64, serde_as};
use sqlx::types::{Uuid, chrono};
use std::time::Duration;
use utoipa::ToSchema;

#[serde_as]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateMessageRequest {
    pub burn_on_read: bool,

    #[schema(value_type = String, format = "base64", example = "AAAAAAAAAAAAAA==")]
    #[serde_as(as = "Base64")]
    pub ciphertext: Vec<u8>,

    #[schema(value_type = String, format = "base64", example = "AAAAAAAAAAAAAAAA")]
    #[serde_as(as = "Base64")]
    pub nonce: [u8; 12],

    #[schema(value_type = String, format = "base64", example = "AAAAAAAAAAAAAAAAAAAAAA==")]
    #[serde_as(as = "Base64")]
    pub salt: [u8; 16],

    #[schema(example = 3600)]
    #[serde(default)]
    pub ttl_seconds: Option<u64>,
}

impl TryFrom<CreateMessageRequest> for Message {
    type Error = MessageError;

    fn try_from(value: CreateMessageRequest) -> Result<Self, Self::Error> {
        let now = chrono::Utc::now();
        Self::builder()
            .created_at(now)
            .expires_at(now + Duration::from_secs(value.ttl_seconds.unwrap_or(3600)))
            .burn_on_read(value.burn_on_read)
            .ciphertext(value.ciphertext)
            .nonce(value.nonce)
            .salt(value.salt)
            .build()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateMessageResponse {
    pub id: Uuid,
}

#[serde_as]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateAttachmentRequest {
    #[schema(value_type = String, format = "base64", example = "AAAAAAAAAAAAAA==")]
    #[serde_as(as = "Base64")]
    pub ciphertext: Vec<u8>,

    #[schema(value_type = String, format = "base64", example = "AAAAAAAAAAAAAAAA")]
    #[serde_as(as = "Base64")]
    pub nonce: [u8; 12],

    #[schema(value_type = String, format = "base64", example = "AAAAAAAAAAAAAAAAAAAAAA==")]
    #[serde_as(as = "Base64")]
    pub salt: [u8; 16],
}

impl CreateAttachmentRequest {
    pub fn into_core(self, message_id: Uuid) -> Result<Attachment, AttachmentError> {
        Attachment::builder()
            .message_id(message_id)
            .ciphertext(self.ciphertext)
            .nonce(self.nonce)
            .salt(self.salt)
            .build()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateAttachmentResponse {
    pub id: Uuid,
}

#[serde_as]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetMessageResponse {
    #[schema(value_type = String, format = "base64", example = "AAAAAAAAAAAAAA==")]
    #[serde_as(as = "Base64")]
    pub ciphertext: Vec<u8>,

    #[schema(value_type = String, format = "base64", example = "AAAAAAAAAAAAAAAA")]
    #[serde_as(as = "Base64")]
    pub nonce: [u8; 12],

    #[schema(value_type = String, format = "base64", example = "AAAAAAAAAAAAAAAAAAAAAA==")]
    #[serde_as(as = "Base64")]
    pub salt: [u8; 16],
}

impl From<Message> for GetMessageResponse {
    fn from(value: Message) -> Self {
        Self {
            ciphertext: value.ciphertext().to_vec(),
            nonce: value.nonce(),
            salt: value.salt(),
        }
    }
}

#[serde_as]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetAttachmentResponse {
    pub id: Uuid,

    #[schema(value_type = String, format = "base64", example = "AAAAAAAAAAAAAA==")]
    #[serde_as(as = "Base64")]
    pub ciphertext: Vec<u8>,

    #[schema(value_type = String, format = "base64", example = "AAAAAAAAAAAAAAAA")]
    #[serde_as(as = "Base64")]
    pub nonce: [u8; 12],

    #[schema(value_type = String, format = "base64", example = "AAAAAAAAAAAAAAAAAAAAAA==")]
    #[serde_as(as = "Base64")]
    pub salt: [u8; 16],
}

impl From<Attachment> for GetAttachmentResponse {
    fn from(value: Attachment) -> Self {
        Self {
            id: value.id(),
            ciphertext: value.ciphertext().to_vec(),
            nonce: value.nonce(),
            salt: value.salt(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetMessageWithAttachmentsResponse {
    pub message: GetMessageResponse,
    pub attachments: Vec<GetAttachmentResponse>,
}

impl From<MessageWithAttachments> for GetMessageWithAttachmentsResponse {
    fn from(value: MessageWithAttachments) -> Self {
        Self {
            message: value.message.into(),
            attachments: value.attachments.into_iter().map(GetAttachmentResponse::from).collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetMessageMetadataResponse {
    pub burn_on_read: bool,
}

impl From<MessageMetadata> for GetMessageMetadataResponse {
    fn from(value: MessageMetadata) -> Self {
        Self {
            burn_on_read: value.burn_on_read,
        }
    }
}
