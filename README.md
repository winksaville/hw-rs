# hw

Simple hw:
```
fn main() {
    println!("Hello, world!");
}
```

Optimized for size in Rust, Cargo.toml with `profile.release`:
```
[package]
name = "hw"
version = "0.1.0"
edition = "2024"

[dependencies]

[profile.release]
opt-level = 'z'
lto = true
codegen-units = 1
panic = "abort"
strip = true
```

Is 300k:
```
$ cargo build --release ; ls -l target/release/hw
   Compiling hw v0.1.0 (/home/wink/data/prgs/hw)
    Finished `release` profile [optimized] target(s) in 2.75s
-rwxr-xr-x 2 wink users 301376 Jan  1 12:41 target/release/hw
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
