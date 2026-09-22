const AES = 'AES-GCM';
const KEY_USAGES: KeyUsage[] = ['encrypt', 'decrypt'];
const PBKDF2 = 'PBKDF2';
const PBKDF2_ITERATIONS = 600000;

export type EncryptionInput = string | Blob | File | ArrayBuffer;
export type EncryptionOutput = {
  ciphertext: Uint8Array;
  nonce: Uint8Array;
  salt: Uint8Array;
  rawKey: Uint8Array | undefined;
};
export type DecryptionOutput = {
  plaintext: ArrayBuffer;
  toText: () => string;
  toBlob: (mimeType?: string) => Blob;
};
export type DerivedKeyResult = {
  key: CryptoKey;
  rawKey: Uint8Array | undefined;
};

async function encode(input: EncryptionInput): Promise<ArrayBuffer> {
  if (input instanceof ArrayBuffer) {
    return input;
  }
  if (input instanceof File || input instanceof Blob) {
    return await input.arrayBuffer();
  }
  return new TextEncoder().encode(input).buffer;
}

async function importRawKey(rawKey: Uint8Array): Promise<CryptoKey> {
  return await crypto.subtle.importKey('raw', rawKey.buffer as ArrayBuffer, AES, false, KEY_USAGES);
}

async function generateRandomKey(): Promise<DerivedKeyResult> {
  const rawKey = crypto.getRandomValues(new Uint8Array(32));
  const key = await importRawKey(rawKey);
  return { key, rawKey };
}

async function derivePasswordKey(password: string, salt: Uint8Array): Promise<DerivedKeyResult> {
  const encoded = new TextEncoder().encode(password);
  const baseKey = await crypto.subtle.importKey('raw', encoded, PBKDF2, false, ['deriveKey']);
  const key = await crypto.subtle.deriveKey(
    {
      name: PBKDF2,
      salt: salt.buffer as ArrayBuffer,
      iterations: PBKDF2_ITERATIONS,
      hash: 'SHA-256',
    },
    baseKey,
    { name: AES, length: 256 },
    false,
    KEY_USAGES,
  );
  return { key, rawKey: undefined };
}

export async function encrypt(
  input: EncryptionInput,
  password?: string,
  rawKey?: Uint8Array,
): Promise<EncryptionOutput> {
  const salt = crypto.getRandomValues(new Uint8Array(16));
  const nonce = crypto.getRandomValues(new Uint8Array(12));
  const encoded = await encode(input);
  let derivedKey: DerivedKeyResult;
  if (password !== undefined) {
    derivedKey = await derivePasswordKey(password, salt);
  } else if (rawKey !== undefined) {
    derivedKey = { key: await importRawKey(rawKey), rawKey };
  } else {
    derivedKey = await generateRandomKey();
  }

  const ciphertext = await crypto.subtle.encrypt({ name: AES, iv: nonce }, derivedKey.key, encoded);
  return {
    ciphertext: new Uint8Array(ciphertext),
    nonce,
    salt,
    rawKey: derivedKey.rawKey,
  };
}

export async function decrypt(
  input: EncryptionOutput,
  password?: string,
): Promise<DecryptionOutput> {
  if (password === undefined && input.rawKey === undefined) {
    throw new Error('rawKey missing');
  }

  const key =
    password === undefined
      ? await importRawKey(input.rawKey!)
      : (await derivePasswordKey(password, input.salt)).key;

  const decrypted = await crypto.subtle.decrypt(
    { name: AES, iv: input.nonce as BufferSource },
    key,
    input.ciphertext as BufferSource,
  );
  return {
    plaintext: decrypted,
    toText: () => new TextDecoder().decode(decrypted),
    toBlob: (mimeType?: string) => new Blob([decrypted], { type: mimeType }),
  };
}
