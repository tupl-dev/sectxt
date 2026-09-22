import { ref } from 'vue';

import { useNotification } from '@/composables/useNotification.ts';

export function useApi() {
  const isLoading = ref<boolean>(false);
  const notis = useNotification();

  const call = async <T>(
    fn: () => Promise<{ data: T | undefined; error: unknown }>,
    fallback?: T,
  ) => {
    isLoading.value = true;
    try {
      const { data, error } = await fn();
      if (data && !error) {
        return data;
      } else {
        notis.error(error);
        return fallback;
      }
    } catch (e) {
      notis.error(e);
      return fallback;
    } finally {
      isLoading.value = false;
    }
  };

  return {
    isLoading,
    call,
  };
}
