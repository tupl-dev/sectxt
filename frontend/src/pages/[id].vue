<template>
  <div class="mx-auto grid w-[50vw] place-items-stretch">
    <template v-if="isLoading">
      <ProgressSpinner />
    </template>
    <template v-else>
      <MessageView v-if="message" :attachments="attachments" :message="message" @copy-message="onCopyMessage" />
    </template>
  </div>
</template>

<script setup lang="ts">
import { toByteArray } from 'base64-js';
import { useConfirm } from 'primevue';
import { onMounted, ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';

import { getMessage, getMetadata } from '@/api/generated';
import MessageView from '@/components/MessageView.vue';
import { useApi } from '@/composables/useApi.ts';
import { useClipboard } from '@/composables/useClipboard.ts';
import { useNotification } from '@/composables/useNotification.ts';
import type { EncryptionOutput } from '@/core/crypto.ts';
import { toEncryptionOutput } from '@/mappers/message.js';

const isLoading = ref<boolean>(false);
const message = ref<EncryptionOutput | undefined>(undefined);
const attachments = ref<EncryptionOutput[]>([]);
const api = useApi();
const clipboard = useClipboard();
const confirm = useConfirm();
const notis = useNotification();
const route = useRoute();
const router = useRouter();

onMounted(async () => {
  const params = route.params as { id: string };
  const id = params.id;
  const hash = route.hash.substring(1);
  await loadData(id, hash ? toByteArray(hash) : undefined);
});

const loadData = async (id: string, rawKey: Uint8Array | undefined) => {
  message.value = undefined;
  attachments.value = [];
  isLoading.value = true;
  try {
    if (!(await promptBurnAfterRead(id))) {
      router.push('/');
      return;
    }

    const response = await api.call(() => getMessage({ path: { id } }));
    if (response !== undefined) {
      message.value = toEncryptionOutput(response.message, rawKey);
      attachments.value = response.attachments.map((attachment) => toEncryptionOutput(attachment, rawKey));
    }
  } catch (e) {
    notis.error(e);
  } finally {
    isLoading.value = false;
  }
};

const onCopyMessage = async (message: string) => {
  if (message !== '') {
    await clipboard.copy(message);
  }
};

const promptBurnAfterRead = async (id: string): Promise<boolean> => {
  const response = await api.call(() => getMetadata({ path: { id } }));
  if (response === undefined) { return false; }
  if (!response.burnOnRead) { return true; }

  return new Promise<boolean>((resolve) => {
    confirm.require({
      message: 'Message will be deleted after reading. Continue?',
      header: 'Confirmation',
      icon: 'pi pi-exclamation-triangle',
      accept: () => resolve(true),
      reject: () => resolve(false),
    });
  });
};
</script>
