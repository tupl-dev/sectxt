import { describe, expect, it } from 'vitest';

import { decrypt, encrypt } from '@/core/crypto.ts';

describe('encrypt', () => {
  const secretText = 'Hello, World!';
  const password = 'password123';

  it('encryption with a password', async () => {
    const sut = await encrypt(secretText, password);

    expect(sut.ciphertext.length).toBeGreaterThan(0);
    expect(sut.nonce.length).toBe(12);
    expect(sut.salt.length).toBe(16);
    expect(sut.rawKey).toBeUndefined();
  });

  it('encryption without a password', async () => {
    const sut = await encrypt(secretText, undefined);

    expect(sut.ciphertext.length).toBeGreaterThan(0);
    expect(sut.nonce.length).toBe(12);
    expect(sut.salt.length).toBe(16);
    expect(sut.rawKey?.length).toBe(32);
  });

  it('encryption produces unique outputs', async () => {
    const sut1 = await encrypt(secretText, password);
    const sut2 = await encrypt(secretText, password);

    expect(sut1.ciphertext).not.toEqual(sut2.ciphertext);
    expect(sut1.nonce).not.toEqual(sut2.nonce);
    expect(sut1.salt).not.toEqual(sut2.salt);
  });

  it('encryption for files', async () => {
    const content = crypto.getRandomValues(new Uint8Array(1024));
    const name = 'test.bin';
    const file = new File([content], name, { type: 'application/octet-stream' });
    const sut = await encrypt(file, password);

    expect(sut.ciphertext.length).toBeGreaterThan(0);
    expect(sut.nonce.length).toBe(12);
    expect(sut.salt.length).toBe(16);
  });
});

describe('decrypt', () => {
  const secretText = 'Hello, World!';
  const password = 'password123';

  it('decryption with a password', async () => {
    const encrypted = await encrypt(secretText, password);
    const sut = await decrypt(encrypted, password);
    expect(sut.toText()).toBe(secretText);
  });

  it('decryption without a password', async () => {
    const encrypted = await encrypt(secretText, undefined);
    const sut = await decrypt(encrypted, undefined);
    expect(sut.toText()).toBe(secretText);
  });

  it('decryption fails with wrong password', async () => {
    const encrypted = await encrypt(secretText, password);
    await expect(decrypt(encrypted, 'wrong_password')).rejects.toThrow();
  });

  it('decryption fails when rawKey is missing in no password mode', async () => {
    const encrypted = await encrypt(secretText, undefined);
    const stripped = {
      ...encrypted,
      rawKey: undefined,
    };
    await expect(decrypt(stripped, undefined)).rejects.toThrow();
  });

  it('decryption for files', async () => {
    const content = crypto.getRandomValues(new Uint8Array(1024));
    const name = 'test.bin';
    const file = new File([content], name, { type: 'application/octet-stream' });
    const encrypted = await encrypt(file, password);
    const sut = await decrypt(encrypted, password);
    const bytes = new Uint8Array(await sut.toBlob('application/octet-stream').arrayBuffer());

    expect(bytes.length).toBe(content.length);
    expect(bytes).toEqual(content);
  });
});
