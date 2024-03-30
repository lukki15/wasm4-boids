# wasm4-boids

A boids flock simulation written in Rust for the [WASM-4](https://wasm4.org) fantasy console.

## Building

Build the cart by running:

```shell
cargo build --release
```

Then run it with:

```shell
w4 run target/wasm32-unknown-unknown/release/cart.wasm
```

For more info about setting up WASM-4, see the [quickstart guide](https://wasm4.org/docs/getting-started/setup?code-lang=rust#quickstart).

## Publish

```shell
cargo build --release

wasm-opt target/wasm32-unknown-unknown/release/cart.wasm -o boids-cart-opt.wasm -Oz --strip-dwarf --strip-producers --zero-filled-memory

w4 bundle boids-cart-opt.wasm --title "wasm-4 boids" --html index.html
```

## Links

- [Documentation](https://wasm4.org/docs): Learn more about WASM-4.
- [Snake Tutorial](https://wasm4.org/docs/tutorials/snake/goal): Learn how to build a complete game
  with a step-by-step tutorial.
- [GitHub](https://github.com/aduros/wasm4): Submit an issue or PR. Contributions are welcome!
