# Vanilla Typescript Wasm Example

A simple typescript example using the wasm npm package for nightrunner-lib.

The simplest way I could find to get a wasm module to work with typescript was to use the [`Vite`](https://vitejs.dev) bundler along with the [`vite-plugin-wasm`](https://github.com/Menci/vite-plugin-wasm?tab=readme-ov-file#usage).

You can accomplish the same results with `webpack` or without any bundler, but the process for doing so is a bit more involved, but the documentation available on the [`wasm-pack`](https://rustwasm.github.io/docs/wasm-pack/) should help you get started.

This example contains the configurations necessary to use the `vite-plugin-wasm` plugin with `vite`, and add
`nightrunner-lib` to your project.

Steps to reproduce this example:

1. Install `vite` and `vite-plugin-wasm`
2. Create a new typescript project to use `nightrunner-lib` with `npm create vite@latest my-app -- --template vanilla-ts`
3. Install `nightrunner-lib` in your project:
   1. From npm: `npm i @nightrunner/nightrunner_lib`
   2. From local folder: add `"nightrunner_lib": "./path/to/lib` to your package.json, under dependencies, and run `npm install` inside the project folder
4. Install the `"vite-plugin-wasm` package: `npm i --save-dev vite-plugin-wasm`
5. Add the `vite-plugin-wasm` plugin to your `vite.config.ts`:
   ```ts
    import { defineConfig } from 'vite'
    import wasm from 'vite-plugin-wasm';
    export default defineConfig({
      plugins: [wasm()],
    })
   ```
6. Import the library in your entry file: `import { NightRunner } from '@nightrunner/nightrunner_lib';`
7. Pass a data object to the NighRunner constructor and start calling the parse function with actions for parsing:
   ```ts
     const nightrunner = new NightRunner(data);
     let result = nightrunner.parse("look");
   ```
8. Run the example: `npm run dev`
