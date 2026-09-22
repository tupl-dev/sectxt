CREATE TABLE "public"."messages" (
  "id" UUID PRIMARY KEY,
  "created_at" TIMESTAMPTZ NOT NULL,
  "expires_at" TIMESTAMPTZ NOT NULL,
  "burn_on_read" BOOL NOT NULL DEFAULT FALSE,
  "ciphertext" BYTEA NOT NULL CHECK (OCTET_LENGTH("ciphertext") > 0),
  "nonce" BYTEA NOT NULL CHECK (OCTET_LENGTH("nonce") = 12),
  "salt" BYTEA NOT NULL CHECK (OCTET_LENGTH("salt") = 16)
);
CREATE INDEX "messages_created_at_idx" ON "public"."messages" ("created_at");
CREATE INDEX "messages_expires_at_idx" ON "public"."messages" ("expires_at");

CREATE TABLE "public"."attachments" (
  "id" UUID PRIMARY KEY,
  "message_id" UUID NOT NULL REFERENCES "public"."messages" ("id") ON DELETE CASCADE,
  "ciphertext" BYTEA NOT NULL CHECK (OCTET_LENGTH("ciphertext") > 0),
  "nonce" BYTEA NOT NULL CHECK (OCTET_LENGTH("nonce") = 12),
  "salt" BYTEA NOT NULL CHECK (OCTET_LENGTH("salt") = 16)
);
CREATE INDEX "attachments_message_id_idx" ON "public"."attachments" ("message_id");

CREATE OR REPLACE VIEW "public"."active_messages" AS
SELECT
  "m"."id",
  "m"."created_at",
  "m"."expires_at",
  "m"."burn_on_read",
  "m"."ciphertext",
  "m"."nonce",
  "m"."salt"
FROM "public"."messages" "m"
WHERE "m"."expires_at" > NOW();
