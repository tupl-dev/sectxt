import path from 'node:path';
import { fileURLToPath, URL } from 'node:url';

import { PrimeVueResolver } from '@primevue/auto-import-resolver';
import tailwindcss from '@tailwindcss/vite';
import vue from '@vitejs/plugin-vue';
import Components from 'unplugin-vue-components/vite';
import { defineConfig } from 'vite';
import vueDevTools from 'vite-plugin-vue-devtools';
import VueRouter from 'vue-router/vite';

export default defineConfig({
  build: {
    emptyOutDir: true,
    outDir: '../dist/frontend/',
  },
  envDir: path.resolve(import.meta.dirname, '../'),
  plugins: [
    VueRouter({}),
    vue(),
    vueDevTools(),
    tailwindcss(),
    Components({
      resolvers: [PrimeVueResolver()],
      types: [{
        from: '@primevue/forms',
        names: ['Form', 'FormField'],
      }],
    }),
  ],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url)),
    },
  },
  server: {
    proxy: {
      '/api': {
        target: 'http://localhost:8080',
        changeOrigin: true,
        secure: false,
      },
    },
  },
});
