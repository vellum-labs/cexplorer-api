import { defineConfig } from "tsup";

export default defineConfig({
  entry: ["src/index.ts"],
  dts: true,
  format: ["esm"],
  minify: true,
  target: "es5",
  clean: true,
  sourcemap: true,
  outDir: "dist",
});
