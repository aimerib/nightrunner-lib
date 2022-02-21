import { defineConfig } from 'vite'
import wasmPack from 'vite-plugin-wasm-pack';
export default defineConfig({
  plugins: [wasmPack([],['@nightrunner/nightrunner_lib'])],
})