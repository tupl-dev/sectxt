import { ref } from 'vue';

import { useNotification } from '@/composables/useNotification.ts';
import { decrypt, encrypt, type EncryptionOutput } from '@/core/crypto.ts';
import type { FileData } from '@/models/attachment.ts';

export function useCrypto() {
  const isLoading = ref<boolean>(false);
  const notis = useNotification();

  const encryptMessage = async (
    message: string,
    password: string | undefined,
  ): Promise<EncryptionOutput | undefined> => {
    isLoading.value = true;
    try {
      return await encrypt(message, password);
    } catch (e) {
      notis.error(e);
    } finally {
      isLoading.value = false;
    }
  };

  const encryptAttachments = async (
    attachments: FileData[],
    password: string | undefined,
    rawKey: Uint8Array | undefined,
  ): Promise<EncryptionOutput[]> => {
    isLoading.value = true;
    try {
      const encrypted = await Promise.all(
        attachments.map((attachment) => encryptAttachment(attachment, password, rawKey)),
      );
      return encrypted.filter((r) => r !== undefined);
    } finally {
      isLoading.value = false;
    }
  };

  const encryptAttachment = async (
    attachment: FileData,
    password: string | undefined,
    rawKey: Uint8Array | undefined,
  ): Promise<EncryptionOutput | undefined> => {
    try {
      return await encrypt(JSON.stringify(attachment), password, rawKey);
    } catch (e) {
      notis.error(e);
    }
  };

  const decryptMessage = async (
    message: EncryptionOutput,
    password: string | undefined,
  ): Promise<string | undefined> => {
    isLoading.value = true;
    try {
      const decrypted = await decrypt(message, password);
      return decrypted.toText();
    } catch {
      notis.error('Failed to decrypt message');
    } finally {
      isLoading.value = false;
    }
  };

  const decryptAttachment = async (
    attachment: EncryptionOutput,
    password: string | undefined,
  ): Promise<FileData> => {
    const decrypted = await decrypt(attachment, password);
    return JSON.parse(decrypted.toText());
  };

  const decryptAttachments = async (
    attachments: EncryptionOutput[],
    password: string | undefined,
  ): Promise<FileData[]> => {
    isLoading.value = true;
    const results = [];
    for (const attachment of attachments) {
      try {
        results.push(await decryptAttachment(attachment, password));
      } catch {
        // ignored
      }
    }
    if (results.length !== attachments.length) {
      notis.error('Failed to decrypt some attachments');
    }
    return results;
  };

  return {
    isLoading,
    encryptMessage,
    encryptAttachments,
    decryptMessage,
    decryptAttachments,
  };
}
