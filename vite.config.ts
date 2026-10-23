import { defineConfig } from "vite";
import { resolve } from "node:path";

export default defineConfig({
  root: "ui",
  clearScreen: false,
  server: { port: 1420, strictPort: true },
  build: {
    outDir: "../dist",
    emptyOutDir: true,
    target: "safari16",
    rollupOptions: {
      input: {
        main: resolve(__dirname, "ui/index.html"),
        toast: resolve(__dirname, "ui/toast.html"),
      },
    },
  },
});
