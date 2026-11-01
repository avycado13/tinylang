# tinylang

A tiny interpreted language written in Rust.

## Run

Requires Rust and Cargo. Save a program as `hello.tl`:

```text
puts "Hello, world!";
```

Then run it with:

```sh
cargo run -- --file hello.tl
```

Programs support variables, arithmetic, `if`/`elsif`/`else`, `while` loops, and functions. Statements end with `;`. Run `cargo run -- --help` for CLI options.
