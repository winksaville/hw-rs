# hw-rs

A minimal "Hello, world!" program in Rust, optimized for binary size.

## Code

```rust
fn main() {
    println!("Hello, world!");
}
```

## Build Configuration

### Cargo.toml

Size optimizations in `profile.release`:
```toml
[profile.release]
opt-level = 'z'
lto = true
codegen-units = 1
panic = "abort"
strip = true
```

### .cargo/config.toml

Two targets are configured:
```toml
# glibc: static + PIE
[target.x86_64-unknown-linux-gnu]
rustflags = [
    "-C", "target-feature=+crt-static"
]

# musl: static + no-PIE (requires system musl: pacman -S musl)
[target.x86_64-unknown-linux-musl]
linker = "musl-gcc"
rustflags = [
    "-C", "relocation-model=static",
    "-C", "link-arg=-no-pie",
]
```

## Compile Instructions

### Prerequisites

Install the required targets:
```bash
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-unknown-linux-musl
```

For musl target, also install musl:
```bash
# Arch Linux
pacman -S musl
```

### Build Commands

Build for glibc (static-pie):
```bash
cargo build --release --target x86_64-unknown-linux-gnu
```

Build for musl (static):
```bash
cargo build --release --target x86_64-unknown-linux-musl
```

## Size Comparison

| Target | Type | Size |
|--------|------|------|
| x86_64-unknown-linux-gnu | ELF 64-bit, static-pie linked | 1,163,184 bytes (1.1 MB) |
| x86_64-unknown-linux-musl | ELF 64-bit, statically linked | 376,776 bytes (368 KB) |

The musl target produces a binary **~3x smaller** than the glibc target.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
