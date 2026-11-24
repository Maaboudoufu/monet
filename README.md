# monet

Monet's website. Pom Poodle mix, Yokohama.

- Rust 1.100, std only — no crates
- Serves `src/html.rs` (single HTML page) + photos from `static/img/`
- Built with orcarouter/Qwen3.8-27B-Uncensored:Q4_K_S on my RTX 3080s

## Run

```sh
cargo build --release
PORT=5170 ./target/release/pompom
```

Open http://localhost:5170

## Build

```sh
cargo test
cargo build --release
```

Images live in `static/img/`. The server reads them from disk at
`<exe>/../../static/img`, so photo swaps don't need a rebuild.
