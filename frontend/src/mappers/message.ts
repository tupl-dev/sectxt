import { fromByteArray, toByteArray } from 'base64-js';

import type { CreateMessageRequest, GetMessageResponse } from '@/api/generated';
import type { EncryptionOutput } from '@/core/crypto.ts';

export function toCreateMessageRequest(
  message: EncryptionOutput,
  burnOnRead: boolean,
  ttlSeconds: number,
): CreateMessageRequest {
  if (message.nonce.length !== 12) {
    throw new Error('nonce must be 12 bytes');
  }
  if (message.salt.length !== 16) {
    throw new Error('salt must be 16 bytes');
  }
  if (message.ciphertext.length === 0) {
    throw new Error('ciphertext must not be empty');
  }
  return {
    burnOnRead: burnOnRead,
    ciphertext: fromByteArray(message.ciphertext),
    nonce: fromByteArray(message.nonce),
    salt: fromByteArray(message.salt),
    ttlSeconds,
  };
}

export function toEncryptionOutput(dto: GetMessageResponse, rawKey: Uint8Array | undefined): EncryptionOutput {
  return {
    ciphertext: toByteArray(dto.ciphertext),
    nonce: toByteArray(dto.nonce),
    salt: toByteArray(dto.salt),
    rawKey,
  };
}
