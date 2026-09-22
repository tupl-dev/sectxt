<template>
  <div class="mx-auto w-[50%]">
    <template v-if="messageUrl">
      <MessageUrlView :message-url="messageUrl" @copy-url="onCopyUrl" @create-new="onCreateNew" />
    </template>
    <template v-else>
      <MessageForm :loading="isLoading" @submit="onSubmit" />
    </template>
  </div>
</template>

<script setup lang="ts">
import { fromByteArray } from 'base64-js';
import { ref } from 'vue';

import {
  createAttachment,
  type CreateAttachmentResponse,
  createMessage,
  type CreateMessageResponse,
} from '@/api/generated';
import MessageForm, { type SubmitPayload } from '@/components/MessageForm.vue';
import MessageUrlView from '@/components/MessageUrlView.vue';
import { useApi } from '@/composables/useApi.ts';
import { useClipboard } from '@/composables/useClipboard.ts';
import { useCrypto } from '@/composables/useCrypto.ts';
import { useNotification } from '@/composables/useNotification.ts';
import { toCreateAttachmentRequest, toFileData } from '@/mappers/attachment.ts';
import { toCreateMessageRequest } from '@/mappers/message.js';

const isLoading = ref<boolean>(false);
const messageUrl = ref<string | undefined>(undefined);
const api = useApi();
const clipboard = useClipboard();
const crypto = useCrypto();
const notis = useNotification();

const submitMessage = async (payload: SubmitPayload) => {
  isLoading.value = true;
  try {
    const encrypted = await crypto.encryptMessage(payload.message ?? '', payload.password);
    if (!encrypted) { return; }
    const request = toCreateMessageRequest(encrypted, payload.burnAfterRead, payload.ttlSeconds);
    const response = await api.call<CreateMessageResponse>(() => createMessage({ body: request }));
    if (response) {
      await uploadAttachments(response.id, payload.attachments, payload.password, encrypted.rawKey);
      messageUrl.value = getMessageUrl(response.id, encrypted.rawKey);
    }
  } catch (e) {
    notis.error(e);
  } finally {
    isLoading.value = false;
  }
};

const uploadAttachments = async (
  messageId: string,
  attachments: File[],
  password: string | undefined,
  rawKey: Uint8Array | undefined,
): Promise<number> => {
  let count = 0;
  try {
    const filesData = await Promise.all(attachments.map((f) => toFileData(f)));
    const encrypted = await crypto.encryptAttachments(filesData, password, rawKey);
    const requests = encrypted.map((e) => toCreateAttachmentRequest(e));
    for (const request of requests) {
      const options = { body: request, path: { message_id: messageId } };
      const response = await api.call<CreateAttachmentResponse>(() => createAttachment(options));
      if (response) { count++; }
    }
  } catch (e) {
    notis.error(e);
  }
  return count;
};

const getMessageUrl = (id: string, rawKey: Uint8Array | undefined): string => {
  if (rawKey) {
    const url = new URL(encodeURIComponent(id), window.location.origin);
    url.hash = fromByteArray(rawKey);
    return url.href;
  } else {
    return new URL(encodeURIComponent(id), window.location.origin).href;
  }
};

const onCopyUrl = async () => {
  await clipboard.copy(messageUrl.value ?? '');
};

const onCreateNew = async () => {
  await clipboard.copy(messageUrl.value ?? '');
  messageUrl.value = undefined;
};

const onSubmit = async (payload: SubmitPayload) => {
  await submitMessage(payload);
};
</script>
