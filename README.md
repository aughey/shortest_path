# shortest_path

Rust implementation of this youtube short: https://www.youtube.com/shorts/KQGBl_vFg58

# Updating Rust

```
rustup default stable
rustup upgrade
rustup component add rustfmt
rustup component add rust-analyzer
```

# Updating dev ci

```
cargo install cargo-binstall
cargo binstall cargo-watch
cargo binstall cargo-nextest
```

# CI

```
cargo watch -x 'nextest r'
```