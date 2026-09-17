import path from "node:path";
import { defineConfig } from "vite";
import viteReact from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";

const rootDir = import.meta.dirname;

export default defineConfig({
  base: "./",
  root: rootDir,
  publicDir: path.join(rootDir, "public"),
  plugins: [
    tailwindcss(),
    viteReact(),
    {
      name: "strip-crossorigin",
      transformIndexHtml(html: string) {
        return html.replaceAll(" crossorigin", "");
      },
    },
  ],
  resolve: {
    alias: { "@": path.join(rootDir, "src") },
  },
  server: {
    host: "127.0.0.1",
    port: 5173,
    strictPort: true,
    open: "/electron/index.html",
  },
  build: {
    outDir: path.join(rootDir, "dist-desktop"),
    emptyOutDir: true,
    sourcemap: false,
    rollupOptions: {
      input: path.join(rootDir, "electron", "index.html"),
    },
  },
});
