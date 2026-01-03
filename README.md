# hw-rs

A "Hello, world!" showdown: how small can a Rust binary get?

This repo contains three variants of the same program, demonstrating the dramatic
size difference between standard library builds and bare-metal `no_std` + `no_main`.

## Size Comparison

| Variant | Binary Size | Relative Size |
|---------|-------------|---------------|
| `std-gnu` | 1,163,184 bytes (1.1 MB) | 996x |
| `std-musl` | 376,776 bytes (368 KB) | 323x |
| `no-std-no-main` | 1,168 bytes (1.1 KB) | 1x |

All three produce the same output: `Hello, world!`

## Variants

### std-gnu

Standard Rust with glibc, statically linked.

```bash
cd std-gnu
cargo build --release
./target/x86_64-unknown-linux-gnu/release/hw-std-gnu
```

### std-musl

Standard Rust with musl libc, statically linked. Requires `musl-gcc`:

```bash
# Arch Linux
pacman -S musl

# Build
cd std-musl
cargo build --release
./target/x86_64-unknown-linux-musl/release/hw-std-musl
```

### no-std-no-main

Bare-metal Rust using direct syscalls. No standard library, no libc.

```bash
cd no-std-no-main
cargo build --profile relver
./target/x86_64-unknown-linux-gnu/relver/hw-no-std-no-main
```

The `relver` profile produces optimized + stripped binaries. Use `cargo build` (dev profile)
for debugging with symbols.

## Prerequisites

Install the required targets:

```bash
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-unknown-linux-musl
```

## Why the Size Difference?

- **std-gnu**: Includes the full Rust standard library plus glibc, statically linked
- **std-musl**: musl is a smaller libc implementation (~3x smaller than glibc)
- **no-std-no-main**: No runtime, no libc, just raw syscalls. The binary contains
  only the machine code needed to write "Hello, world!" to stdout and exit.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
