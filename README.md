Rust + Typescript WebGl Demo
=========================

Just wanted to test some web gl stuff with web assembly & typescript.
Originally I wanted to use glitch.com but they don't support rust beyond
cargo build without any dependencies at the moment (but their support
pages straight up say they don't support it).

## Running Locally

### Dependencies

- [`Cargo`][cargo] to build rust
- [`wasm-pack`][wasmpack] for wasm things
- Either [Yarn], [npm], or whatever js package manager, although
  you may want to update the script for starting to use your package
  manager, if you're not using yarn.

[npm]: https://www.npmjs.com/
[Yarn]: https://yarnpkg.com/en/
[cargo]: https://crates.io/
[wasmpack]: https://github.com/rustwasm/wasm-pack

## Project Layout

### TypeScript + CSS, DOM wrapper.

Anyways there is a light typescript wrapper around the web assembly portion
that gets the canvas, context, shaders, and tracks bounds of the canvas. So
you could techinically drop the canvas into another application and it'll
scale to the window bounds it is placed in, but that is kinda incidential.
All this lives in `src`.

### Shader Location.

Since I use webpack to make bundling the application alot simpler to avoid
having to use ajax calls to get the shader source or do that hacky thing
were you put the shader in a sciprt tag. I can just import the shaders as
normal modules. The shaders live in `src/shaders`.

### Rust Core.

Rust portion all lives in `runtime`, and the bulk of the logic takes place
in here. Bindings to the binary are generated with `wasm-bindgen`.
