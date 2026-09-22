<template>
  <div class="flex flex-col">
    <FloatLabel variant="on">
      <Textarea
        v-model="decryptedMessage"
        fluid
        input-id="message"
        readonly
        rows="10"
        style="resize: none"
        @click="emit('copy-message', decryptedMessage ?? '')"
      />
      <label for="message">Message</label>
    </FloatLabel>

    <div v-if="decryptedAttachments.length > 0" class="flex flex-col gap-2">
      <AttachmentCard
        v-for="(attachment, idx) in decryptedAttachments"
        :key="idx"
        :name="attachment.name"
        :size="attachment.size"
        @download="onDownload(attachment)"
      />
    </div>

    <div v-if="!props.message.rawKey && decryptedMessage === undefined">
      <FloatLabel variant="on">
        <Password v-model="password" class="mt-4" fluid input-id="password" toggle-mask />
        <label for="password">Decryption Password</label>
      </FloatLabel>
      <Button class="mt-4 justify-self-start" label="Decrypt" @click="onDecrypt" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { toByteArray } from 'base64-js';
import { onMounted, ref } from 'vue';

import AttachmentCard from '@/components/AttachmentCard.vue';
import { useCrypto } from '@/composables/useCrypto.ts';
import type { EncryptionOutput } from '@/core/crypto.ts';
import type { FileData } from '@/models/attachment.ts';

const props = defineProps<{
  message: EncryptionOutput;
  attachments: EncryptionOutput[];
}>();
const emit = defineEmits<{
  (e: 'copy-message', message: string): void;
}>();

const password = ref<string | undefined>(undefined);
const decryptedMessage = ref<string | undefined>(undefined);
const decryptedAttachments = ref<FileData[]>([]);
const crypto = useCrypto();

onMounted(async () => {
  if (props.message.rawKey) {
    await onDecrypt();
  }
});

const onDecrypt = async () => {
  decryptedMessage.value = await crypto.decryptMessage(props.message, password.value);
  decryptedAttachments.value = await crypto.decryptAttachments(props.attachments, password.value);
};

const onDownload = async (attachment: FileData) => {
  const bytes = toByteArray(attachment.base64);
  const blob = new Blob([bytes.buffer as ArrayBuffer], { type: attachment.type });
  const url = window.URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = attachment.name;
  a.click();
  window.URL.revokeObjectURL(url);
};
</script>
