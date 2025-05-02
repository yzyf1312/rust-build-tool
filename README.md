# Rust Build Tool

A powerful tool for building optimized Rust executables with additional features like dependency checking.

## Features

- 🚀 Optimized builds with LTO and maximum optimization
- 🔍 Detect and remove unused dependencies
- 🗜️ UPX compression support
- 🖥️ Cross-platform support
- ⚡ Fast builds with sensible defaults

## Requirements

- Rust nightly toolchain
- cargo-udeps (for dependency checking)
- UPX (optional, for compression)

## Installation

```bash
cargo install --path .
```

To build and run directly from source with UPX compression:
```bash
cargo run -- build --upx
```

## Usage

### Build Command

```bash
rust_build_tool build [OPTIONS]
```

Options:
- `--target`: Specify target platform (default: auto-detect)
- `--upx`: Enable UPX compression
- `--clean`: Clean before building

Example:
```bash
rust_build_tool build --target x86_64-unknown-linux-gnu --upx
```

### Dependency Check

```bash
rust_build_tool depcheck
```

This will scan for unused dependencies and prompt for removal confirmation.

## Configuration

The tool automatically configures these release profile settings:
- opt-level = 'z'
- lto = true
- codegen-units = 1
- panic = 'abort'
- strip = true

## Examples

1. Simple build:
```bash
rust_build_tool build
```

2. Build with UPX compression:
```bash
rust_build_tool build --upx
```

3. Check and remove unused dependencies:
```bash
rust_build_tool depcheck
```

## Best Practices

- For maximum optimization, use the `--upx` flag to compress the final executable
- Regularly run `depcheck` to keep your dependencies clean
- For cross-compiling, explicitly specify the target with `--target`
- The tool automatically applies optimal release profile settings
- Consider using this tool in CI/CD pipelines for consistent builds

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Open a pull request
