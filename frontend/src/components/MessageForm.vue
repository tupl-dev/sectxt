<template>
  <Form class="flex flex-col gap-4" @submit="onSubmit">
    <FloatLabel class="w-full" variant="on">
      <Textarea v-model="message" fluid input-id="message" rows="10" style="resize: none" />
      <label for="message">Message</label>
    </FloatLabel>

    <FileUpload
      choose-label="Attachment"
      :file-limit="MAX_FILE_LIMIT"
      :max-file-size="MAX_FILE_SIZE"
      mode="advanced"
      multiple
      :show-cancel-button="false"
      :show-upload-button="false"
      @remove="onFileRemove"
      @select="onFileSelect"
    />

    <div class="flex gap-4">
      <FloatLabel class="w-full" variant="on">
        <Password v-model="password" class="w-full" fluid input-id="password" toggle-mask />
        <label for="password">Password (Optional)</label>
      </FloatLabel>

      <FloatLabel variant="on">
        <Select v-model="ttlSeconds" fluid input-id="options" option-label="name" option-value="value" :options="TTL_OPTIONS" />
        <label for="options">Time to Live</label>
      </FloatLabel>
    </div>

    <div class="flex w-full items-center justify-between">
      <Button icon="pi pi-check" label="Create" :loading="props.loading" severity="success" type="submit" />
      <div class="flex items-center gap-2">
        <Checkbox v-model="burnAfterRead" binary input-id="burn" />
        <label for="burn">Burn After Reading</label>
      </div>
    </div>
  </Form>
</template>

<script setup lang="ts">
import type { FileUploadRemoveEvent, FileUploadSelectEvent } from 'primevue';
import { ref } from 'vue';

export type SubmitPayload = {
  message: string | undefined;
  password: string | undefined;
  burnAfterRead: boolean;
  ttlSeconds: number;
  attachments: File[];
};

const props = defineProps<{
  loading: boolean;
}>();
const emit = defineEmits<{
  (e: 'submit', payload: SubmitPayload): void;
}>();

const ONE_HOUR = 60 * 60;
const MAX_FILE_LIMIT = 10;
const MAX_FILE_SIZE = 15 * 1024 * 1024;
const TTL_OPTIONS = [
  { name: '30 minutes', value: ONE_HOUR / 2 },
  { name: '1 hour', value: ONE_HOUR },
  { name: '1 day', value: 24 * ONE_HOUR },
  { name: '7 days', value: 7 * 24 * ONE_HOUR },
  { name: '30 days', value: 30 * 24 * ONE_HOUR },
];

const message = ref<string | undefined>(undefined);
const password = ref<string | undefined>(undefined);
const burnAfterRead = ref<boolean>(false);
const attachments = ref<File[]>([]);
const ttlSeconds = ref<number>(ONE_HOUR);

const onFileRemove = (event: FileUploadRemoveEvent) => {
  attachments.value = event.files;
};

const onFileSelect = (event: FileUploadSelectEvent) => {
  attachments.value = event.files;
};

const onSubmit = async () => {
  emit('submit', {
    message: message.value,
    password: password.value,
    burnAfterRead: burnAfterRead.value,
    ttlSeconds: ttlSeconds.value,
    attachments: attachments.value,
  });
};
</script>
