import { defineConfig } from 'vite';
import { resolve } from 'path';

export default defineConfig({
    root: 'src',
    base: './',
    resolve: {
        alias: {
            '@tauri-apps': resolve(__dirname, 'node_modules/@tauri-apps'),
        },
    },
    build: {
        outDir: '../dist',
        emptyOutDir: true,
    },
    server: {
        fs: {
            strict: true,
        },
    },
});
