
## Nightly Rust

Use `rustup`

```
rustup default nightly
```

## Postgres Libraries

Install postgres libraries. Avoid installing the whole database. We'll be 
running that in a container.

```
sudo pacman -S postgresql-libs
```

## Diesel CLI

Diesel is our migration framework. Install with only Postgres features to avoid
needing MySQL libraries.

```
cargo install diesel_cli --no-default-features --features postgres
```
