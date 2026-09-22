import { fromByteArray } from 'base64-js';
import { describe, expect, it } from 'vitest';

import type { GetAttachmentResponse } from '@/api/generated';
import { toCreateAttachmentRequest, toFileData } from '@/mappers/attachment.ts';
import { toEncryptionOutput } from '@/mappers/attachment.ts';

describe('toCreateAttachmentRequest', () => {
  it('creates a dto', () => {
    const attachment = {
      ciphertext: new Uint8Array(10),
      nonce: new Uint8Array(12),
      salt: new Uint8Array(16),
      rawKey: new Uint8Array(32),
    };

    const sut = toCreateAttachmentRequest(attachment);
    expect(sut).toEqual({
      ciphertext: 'AAAAAAAAAAAAAA==',
      nonce: 'AAAAAAAAAAAAAAAA',
      salt: 'AAAAAAAAAAAAAAAAAAAAAA==',
    });
  });

  it('throw on invalid nonce', () => {
    const attachment = {
      ciphertext: new Uint8Array(10),
      nonce: new Uint8Array(3),
      salt: new Uint8Array(16),
      rawKey: new Uint8Array(32),
    };
    expect(() => toCreateAttachmentRequest(attachment)).toThrow();
  });

  it('throw on invalid salt', () => {
    const attachment = {
      ciphertext: new Uint8Array(10),
      nonce: new Uint8Array(12),
      salt: new Uint8Array(3),
      rawKey: new Uint8Array(32),
    };
    expect(() => toCreateAttachmentRequest(attachment)).toThrow();
  });

  it('throw on invalid ciphertext', () => {
    const attachment = {
      ciphertext: new Uint8Array(0),
      nonce: new Uint8Array(12),
      salt: new Uint8Array(16),
      rawKey: new Uint8Array(32),
    };
    expect(() => toCreateAttachmentRequest(attachment)).toThrow();
  });
});

describe('toEncryptionOutput', () => {
  it('creates an encrypted attachment', () => {
    const dto: GetAttachmentResponse = {
      id: '',
      ciphertext: 'AAAAAAAAAAAAAA==',
      nonce: 'AAAAAAAAAAAAAAAA',
      salt: 'AAAAAAAAAAAAAAAAAAAAAA==',
    };

    const sut = toEncryptionOutput(dto, undefined);
    expect(sut).toEqual({
      ciphertext: new Uint8Array(10),
      nonce: new Uint8Array(12),
      salt: new Uint8Array(16),
    });
  });

  it('creates an encrypted attachment (no password)', () => {
    const dto: GetAttachmentResponse = {
      id: '',
      ciphertext: 'AAAAAAAAAAAAAA==',
      nonce: 'AAAAAAAAAAAAAAAA',
      salt: 'AAAAAAAAAAAAAAAAAAAAAA==',
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

describe('toFileData', () => {
  it('converts text file', async () => {
    const content = 'hello world!';
    const name = 'test.txt';
    const sut = new File([content], name, { type: 'text/plain' });
    const result = await toFileData(sut);
    expect(result).toEqual({
      base64: btoa(content),
      name,
      size: sut.size,
      type: 'text/plain',
    });
  });

  it('converts binary file', async () => {
    const content = crypto.getRandomValues(new Uint8Array(1024));
    const name = 'test.bin';
    const sut = new File([content], name, { type: 'application/octet-stream' });
    const result = await toFileData(sut);
    expect(result).toEqual({
      base64: fromByteArray(content),
      name,
      size: sut.size,
      type: 'application/octet-stream',
    });
  });
});
