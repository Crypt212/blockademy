import { fileURLToPath, URL } from 'url';
import environment from 'vite-plugin-environment';
import { defineConfig } from 'vite'
import dotenv from 'dotenv';
import react from '@vitejs/plugin-react'

// https://vite.dev/config/
export default defineConfig({
    build: {
        emptyOutDir: true,
    },
    optimizeDeps: {
        esbuildOptions: {
            define: {
                global: "globalThis",
            },
        },
    },
    server: {
        proxy: {
            "/api": {
                target: "http://127.0.0.1:4943",
                changeOrigin: true,
            },
        },
    },
    plugins: [
        react(),
        environment("all", { prefix: "CANISTER_" }),
        environment("all", { prefix: "DFX_" }),
    ],
    resolve: {
        alias: [
            {
                find: "declarations",
                replacement: fileURLToPath(
                    new URL("../declarations", import.meta.url)
                ),
            },
        ],
        dedupe: ['@dfinity/agent'],
    },
})

dotenv.config({ path: '../../.env' });
