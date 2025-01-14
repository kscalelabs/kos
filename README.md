# K-Scale OS

Welcome to the K-Scale Operating System!

## Building

### Prerequisites

- `cross` toolchain

You can install `cross` by running:

```bash
cargo install cross
```

### Native build

Native build:

```bash
cargo build
```

You can run the `kos-stub` endpoint for testing purposes using the command:

```bash
cargo run --bin kos-stub
```

After doing this, you can test the endpoint using the `pykos` client:

```python
import pykos

client = pykos.KOS("127.0.0.1")
```

### Cross build

Cross build for `kbot`:

```bash
cross build --release --target aarch64-unknown-linux-gnu
```

## Running

```bash
RUST_LOG=debug cargo run
```

You can specify logging levels for individual modules by adding `module_name=log_level` to the `RUST_LOG` environment variable. For example:

```bash
RUST_LOG=debug,krec=warn cargo run
```

### List of features (--features / -F flag)

Features are how you specify the specific platform to run K-OS on (e.g. -F kos-kbot when running on K-Bot)

- kos-kbot
- zeroth-01 (not finished)
- sim (not finished)

## Adding a new embodiment

Reference the existing platforms / features in [platforms](platforms).

You essentially create another package (Cargo.toml, lib.rs, etc) with the necessary actuator and imu implementations according to the specifications in [kos-core](kos-core/src/services)

To save trace logs to a file, pass the `--log` flag:

```bash
cargo run -- --log
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
