import { defineConfig } from 'vite';
import { sveltekit } from '@sveltejs/kit/vite';

export default defineConfig({
  plugins: [sveltekit()],
  resolve: {
    alias: {
      // '@oicl/openbridge-webcomponents': '/node_modules/@oicl/openbridge-webcomponents',
    }
  },
  build: {
    target: ['esnext'],
    minify: process.env.TAURI_DEBUG ? false : 'esbuild',
    sourcemap: !!process.env.TAURI_DEBUG,
  }
});
