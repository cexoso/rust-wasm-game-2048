import wasm from "vite-plugin-wasm";
import { defineConfig } from "vite";
import { vanillaExtractPlugin } from "@vanilla-extract/vite-plugin";

export default defineConfig({
  plugins: [wasm(), vanillaExtractPlugin()],
});
