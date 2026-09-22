import { describe, expect, it } from 'vitest';

import type { CreateMessageRequest } from '@/api/generated';
import { toCreateMessageRequest, toEncryptionOutput } from '@/mappers/message.ts';

describe('toCreateMessageRequest', () => {
  it('creates a dto', () => {
    const message = {
      ciphertext: new Uint8Array(10),
      nonce: new Uint8Array(12),
      salt: new Uint8Array(16),
      rawKey: new Uint8Array(32),
    };

    const sut = toCreateMessageRequest(message, true, 60);
    expect(sut).toEqual({
      burnOnRead: true,
      ciphertext: 'AAAAAAAAAAAAAA==',
      nonce: 'AAAAAAAAAAAAAAAA',
      salt: 'AAAAAAAAAAAAAAAAAAAAAA==',
      ttlSeconds: 60,
    });
  });

  it('throw on invalid nonce', () => {
    const message = {
      ciphertext: new Uint8Array(10),
      nonce: new Uint8Array(3),
      salt: new Uint8Array(16),
      rawKey: new Uint8Array(32),
    };
    expect(() => toCreateMessageRequest(message, true, 60)).toThrow();
  });

  it('throw on invalid salt', () => {
    const message = {
      ciphertext: new Uint8Array(10),
      nonce: new Uint8Array(12),
      salt: new Uint8Array(3),
      rawKey: new Uint8Array(32),
    };
    expect(() => toCreateMessageRequest(message, true, 60)).toThrow();
  });

  it('throw on invalid ciphertext', () => {
    const message = {
      ciphertext: new Uint8Array(0),
      nonce: new Uint8Array(12),
      salt: new Uint8Array(16),
      rawKey: new Uint8Array(32),
    };
    expect(() => toCreateMessageRequest(message, true, 60)).toThrow();
  });
});

describe('toEncryptionOutput', () => {
  it('creates an encrypted message', () => {
    const dto: CreateMessageRequest = {
      burnOnRead: true,
      ciphertext: 'AAAAAAAAAAAAAA==',
      nonce: 'AAAAAAAAAAAAAAAA',
      salt: 'AAAAAAAAAAAAAAAAAAAAAA==',
      ttlSeconds: 60,
    };

    const sut = toEncryptionOutput(dto, undefined);
    expect(sut).toEqual({
      ciphertext: new Uint8Array(10),
      nonce: new Uint8Array(12),
      salt: new Uint8Array(16),
    });
  });

  it('creates an encrypted message (no password)', () => {
    const dto: CreateMessageRequest = {
      burnOnRead: true,
      ciphertext: 'AAAAAAAAAAAAAA==',
      nonce: 'AAAAAAAAAAAAAAAA',
      salt: 'AAAAAAAAAAAAAAAAAAAAAA==',
      ttlSeconds: 60,
    };

    const sut = toEncryptionOutput(dto, new Uint8Array(32));
    expect(sut).toEqual({
      ciphertext: new Uint8Array(10),
      nonce: new Uint8Array(12),
      salt: new Uint8Array(16),
      rawKey: new Uint8Array(32),
    });
  });
});
