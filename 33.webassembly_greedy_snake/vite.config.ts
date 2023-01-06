import { defineConfig } from 'vite'

export default defineConfig({
  root: 'www',
  build: {
    target: 'esnext',
    outDir: '../dist',
  },
})
