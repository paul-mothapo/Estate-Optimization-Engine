import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import tailwindcss from '@tailwindcss/vite'; // Import the plugin

// https://vite.dev/config/
export default defineConfig({
  base: '/web/',
  plugins: [react(), tailwindcss()],
  server: {
    proxy: {
      '/v1': {
        target: 'http://127.0.0.1:8080',
        changeOrigin: true,
      },
      '/health': {
        target: 'http://127.0.0.1:8080',
        changeOrigin: true,
      },
      '/api-docs': {
        target: 'http://127.0.0.1:8080',
        changeOrigin: true,
      },
    },
  },
})
