import { useNotification } from '@/composables/useNotification.ts';

export function useClipboard() {
  const notis = useNotification();

  const copy = async (text: string, message: string = 'Copied to clipboard') => {
    await navigator.clipboard.writeText(text);
    notis.info(message);
  };

  return {
    copy,
  };
}
