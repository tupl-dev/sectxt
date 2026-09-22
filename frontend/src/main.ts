import 'primeicons/primeicons.css';
import './style.css';

import Aura from '@primeuix/themes/aura';
import PrimeVue from 'primevue/config';
import ConfirmationService from 'primevue/confirmationservice';
import ToastService from 'primevue/toastservice';
import Tooltip from 'primevue/tooltip';
import { createApp } from 'vue';
import { createRouter, createWebHistory } from 'vue-router';
import { routes } from 'vue-router/auto-routes';

import { client } from '@/api/generated/client.gen.ts';

import App from './App.vue';

const app = createApp(App);
app.use(
  createRouter({
    history: createWebHistory(),
    routes,
  }),
);

app.use(PrimeVue, {
  theme: {
    preset: Aura,
    options: {
      darkModeSelector: '.dark-mode',
      cssLayer: false,
    },
  },
});

client.setConfig({
  baseUrl: import.meta.env.VITE_API_BASE_URL,
});

app.use(ConfirmationService);
app.use(ToastService);
app.directive('tooltip', Tooltip);
app.mount('#app');
