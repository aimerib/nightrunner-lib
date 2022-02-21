# Vanilla Typescript Wasm Example

A simple typescript example using the wasm npm package for nightrunner-lib.

The simplest way I could find to get a wasm module to work with typescript was to use the [`Vite`](https://vitejs.dev) bundler along with the [`vite-plugin-wasm-pack`](https://github.com/nshen/vite-plugin-wasm-pack#use-wasm-pack-package-installed-via-npm).

You can accomplish the same results with `webpack` or without any bundler, but the process for doing so is a bit more involved, but the documentation available on the [`wasm-pack`](https://rustwasm.github.io/docs/wasm-pack/) should help you get started.

This example contains the configurations necessary to use the `vite-plugin-wasm-pack` plugin with `vite`, and add
`nightrunner-lib` to your project.

Steps to reproduce this example:

1. Install `vite` and `vite-plugin-wasm-pack`
2. Create a new typescript project to use `nightrunner-lib` with `yarn create vite my-app --template vanilla-ts`
3. Install `nightrunner-lib` in your project:
   1. From npm: `yarn add @nightrunner/nightrunner_lib`
   2. From local folder: add `"nightrunner_lib": "./path/to/lib` to your package.json, under dependencies, and run `yarn` inside the project folder
4. Install the `"vite-plugin-wasm-pack` package: `yarn add vite-plugin-wasm-pack`
5. Add the `vite-plugin-wasm-pack` plugin to your `vite.config.ts` file and add the `nightrunner-lib` as an npm dependency:
   ```
    import { defineConfig } from 'vite'
    import wasmPack from 'vite-plugin-wasm-pack';
    export default defineConfig({
      plugins: [wasmPack([],['@nightrunner/nightrunner_lib'])],
    })
   ```
   Note: The first set of brackets in the `wasmPack` plugin call is for local packages, and the second set is for npm packages. Refer to the documentation for more information.
6. Import the library in your entry file: `import init, {NightRunner} from '@nightrunner/nightrunner_lib';` - Here the init function will load the wasm module and bring the library into scope.
7. Run the init() and resolve the promise to get the library into scope. Vite has global async/await support turned on by default, so you can simply use `await init()` to get the library into scope.
8. Pass a data object to the NighRunner constructor and start calling the parse function with actions for parsing:
   ```
     const nightrunner = new NightRunner(data);
     let result = nr.parse("look");
   ```
9. Run the example: `yarn dev`
