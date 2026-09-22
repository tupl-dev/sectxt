<template>
  <Card>
    <template #content>
      <div class="flex items-center justify-between">
        <div class="flex flex-col gap-1">
          <span class="text-lg font-bold">{{ props.name }}</span>
          <span class="text-sm text-gray-400">{{ `Size: ${(props.size / 1000).toFixed(2)} KB` }}</span>
        </div>

        <Button
          v-tooltip="'Download'"
          aria-label="Download"
          icon="pi pi-download"
          rounded
          text
          @click="emit('download')"
        />
      </div>
    </template>
  </Card>
</template>

<script setup lang="ts">
const props = defineProps<{
  name: string;
  size: number;
}>();

const emit = defineEmits<{
  (e: 'download'): void;
}>();
</script>
