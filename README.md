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
- 
---

## How to Run

### Part 1
Requires WABT and Node.js installed.
```bash
cd Part_1
wat2wasm recursive.wat -o recursive.wasm
node run.js
```

### Part 2 & 3 (wasm-is-prime)
Requires nightly Rust, wasm-pack, Node.js v16 via nvm.

**First time setup:**
```bash
nvm use 16
cd wasm-is-prime
wasm-pack build
cd www
npm install
```

**Run the app:**
```bash
cd wasm-is-prime/www
nvm use 16
npm run start
```
Navigate to `http://localhost:8080` in your browser.

**If you change any Rust code in `src/lib.rs`, rebuild first:**
```bash
cd wasm-is-prime
wasm-pack build
cd www
npm install
npm run start
```
