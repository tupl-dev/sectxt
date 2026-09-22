<template>
  <div class="flex flex-col gap-2">
    <FloatLabel variant="on">
      <InputText fluid input-id="response" :model-value="props.messageUrl" readonly @click="onCopyUrl" />
      <label for="response">Message URL</label>
    </FloatLabel>

    <Button class="w-fit self-start" label="Copy & Create New" @click="onCreateNew" />
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  messageUrl: string;
}>();
const emit = defineEmits<{
  (e: 'create-new'): void;
  (e: 'copy-url'): void;
}>();

const onCreateNew = () => {
  emit('create-new');
};

const onCopyUrl = () => {
  emit('copy-url');
};
</script>
