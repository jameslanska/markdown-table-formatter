# Publishing to NPM

Install `wasm-pack`

```shell
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

Build the project

- requires Rust >= 1.65
- installs `wasm32-unknown-unknown` target via `rustup`
- compiles the Rust sources into a WebAssembly `.wasm` binary via `cargo`
- uses `wasm-bindgen` to generate the JavaScript API for using the Rust-generated WebAssembly

```shell
cd ~/project_root
wasm-pack build --target nodejs --scope jameslanska
```

When the build has completed, its artifacts can be found in the `pkg` directory with the following structure

```text
pkg/
├── README.md
├── package.json
├── table_formatter.d.ts
├── table_formatter.js
├── table_formatter_bg.js
├── table_formatter_bg.wasm
└── table_formatter_bg.wasm.d.ts
```

Publish to [npmjs.com](https://www.npmjs.com)

```shell
wasm-pack publish
```
