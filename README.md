# K-Scale OS

Welcome to the K-Scale Operating System!

## Building

### Prerequisites

- `cross` toolchain

### Native build
Native build with stub features:
```bash
cargo build --features stub
```

### Cross build
Cross build for kbot:
```bash
cross build --release --target aarch64-unknown-linux-gnu --features kscale_pro
```

## Running

```bash
RUST_LOG=debug cargo run --features stub
```

You can specify logging levels for individual modules by adding `module_name=log_level` to the `RUST_LOG` environment variable. For example:
```bash
RUST_LOG=debug,krec=warn cargo run --features stub
```

To save trace logs to a file, pass the `--log` flag:
```bash
cargo run --features stub -- --log
```

## Contributing
- Use `cargo fmt --all` to format the code.
- Use `cargo clippy` to check for lint errors.
- Use `cargo test` to run the tests.
- Use `tracing` for logging.
- Use `eyre` to handle errors.
- No `unwrap()` or `expect()`.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
