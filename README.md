# 🦀🕸 `yew-wasm-worker-template`

[![Build Status](https://travis-ci.org/VictorGavrish/rust-wasm-worker-template.svg?branch=master)](https://travis-ci.org/VictorGavrish/rust-wasm-worker-template)

> **Kickstart your Rust, WebAssembly, Webpack, Yew, and Web Worker project!**

This template is designed for creating monorepo-style Web applications with
Rust-generated WebAssembly running inside a Web Worker and Webpack without
publishing your wasm to NPM. This template uses Yew as the frontend and for
launching the workers.

## 🔋 Batteries Included

This template comes pre-configured with all the boilerplate for compiling Rust
to WebAssembly and hooking into a Webpack build pipeline.

- `npm run start` -- Serve the project locally for development at
  `http://localhost:8080`.

- `npm run build` -- Bundle the project (in production mode).

## 🚴 Using This Template

This template shows how it's possible to use wasm-bindegen to generate a single
wasm file for both the the frontend and for workers - simplifying build tremendously.
