import { useToast } from 'primevue';

export function useNotification() {
  const toast = useToast();

  const error = (error: unknown, summary: string = 'Error') => {
    console.error(error);

    let message = 'An unknown error occurred';
    if (error instanceof Error) {
      message = error.message;
    } else if (typeof error === 'string') {
      message = error;
    } else if (error && typeof error === 'object' && 'message' in error) {
      message = String((error as { message: unknown }).message);
    }
    toast.add({ severity: 'error', summary, detail: message, life: 5000 });
  };

  const info = (message: string, summary: string = 'Info') => {
    toast.add({ severity: 'info', summary, detail: message, life: 5000 });
  };

  const success = (message: string, summary: string = 'Success') => {
    toast.add({ severity: 'success', summary, detail: message, life: 5000 });
  };

  return {
    error,
    info,
    success,
  };
}
