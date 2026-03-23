# ECE 421 - Lab 9: Rust and WebAssembly

## Part 1: WebAssembly Text Format
A recursive WAT function `RecursiveCount(n)` that sums all integers from n up to 10.
- Example: `RecursiveCount(7) = 7+8+9+10 = 34`
- Compiled and tested using WABT and Node.js locally

## Part 2: Hello World with Rust + WASM
A basic web page that displays a "Hello, wasm-is-prime!" alert using Rust compiled to WebAssembly via wasm-pack.

## Part 3: Is Prime App
A web application that takes an integer input and checks if it is prime.
- Primality logic written in Rust
- Compiled to WebAssembly using wasm-pack
- Result displayed visually on an HTML canvas with a green checkmark for prime and a red x-mark for not prime

## Setup Notes
- Requires nightly Rust with MVP target CPU flag due to webpack 4 compatibility issue with Rust 1.82+
- Requires Node.js v16 via nvm due to webpack-dev-server compatibility
