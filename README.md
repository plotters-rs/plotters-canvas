# plotters-canvas - The HTML5 canvas backend for Plotters

This is a part of plotters project. For more details, please check the following links:

- For high-level intro of Plotters, see: [Plotters on crates.io](https://crates.io/crates/plotters)
- Check the main repo at [Plotters repo](https://github.com/38/plotters.git)
- For detailed documentation about this crate, check [plotters-backend on docs.rs](https://docs.rs/plotters-backend/)
- You can also visit Plotters [Homepage](https://plotters-rs.github.io)

## Testing 

This crate needs to be tested in a browser environment. In order to test, you should install `wasm-pack` first.

```bash
cargo install wasm-pack
```

To run the test case with wasm-pack. You need choose what browser you want to run the test cases. You should also be able
to use `--chrome` or `--safari` as well.

```bash
wasm-pack test --firefox  
```

Also you should be able to run it headlessly by adding the headless param to the testing command.

```bash
wasm-pack test --firefox --headless
```
