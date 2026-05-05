# Toolchain setup — Rust + WebAssembly

This document records exactly what was installed on this machine and where it lives, so the setup is reproducible and reversible.

## What got installed

| Component | Version | Location |
| --- | --- | --- |
| `rustup` | latest | `~/.rustup/`, `~/.cargo/bin/rustup` |
| `rustc` (stable) | 1.95.0 | `~/.cargo/bin/rustc` |
| `cargo` | 1.95.0 | `~/.cargo/bin/cargo` |
| `wasm-pack` | 0.14.0 | `~/.cargo/bin/wasm-pack` |
| `wasm32-unknown-unknown` target | — | `~/.rustup/toolchains/stable-*/lib/rustlib/wasm32-unknown-unknown/` (added on first build) |

Total disk usage after `wasm-pack` is built: roughly 1.0–1.5 GB under `~/.rustup` and `~/.cargo` combined.

## The exact commands that were run

### 1. Install rustup + stable Rust

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable --profile default
```

Flags:
- `-y` — accept all defaults non-interactively.
- `--default-toolchain stable` — install the stable channel as the default toolchain.
- `--profile default` — installs `rustc`, `cargo`, `rust-std`, `rustfmt`, `clippy`, and docs.

What this changes outside `~/.rustup` and `~/.cargo`:
- Appends a line to `~/.zshrc` that sources `~/.cargo/env`, putting `~/.cargo/bin` on `PATH` for new shells.

### 2. Make `cargo` available in the current shell

```sh
. "$HOME/.cargo/env"
```

This is only needed for the shell session in which rustup was just installed. New terminal windows pick it up via `~/.zshrc`.

### 3. Install wasm-pack

```sh
cargo install wasm-pack
```

`wasm-pack` is the build tool that compiles a Rust crate to a `.wasm` module and emits a JS/TS package wrapping it. It is not strictly required (`cargo build --target wasm32-unknown-unknown` works directly), but it handles `wasm-bindgen` post-processing, optimization (`wasm-opt`), and packaging in one step.

### 4. wasm32 target

The first time `wasm-pack build` runs, it adds the `wasm32-unknown-unknown` target automatically via `rustup target add wasm32-unknown-unknown`. No explicit step needed.

## Verifying

```sh
rustc --version    # rustc 1.95.0 (...)
cargo --version    # cargo 1.95.0 (...)
wasm-pack --version  # wasm-pack 0.14.0
```

## Updating

```sh
rustup update              # updates stable toolchain
cargo install wasm-pack    # re-runs install, replaces older binary
```

## Uninstalling

Fully reversible:

```sh
rustup self uninstall      # removes ~/.rustup, ~/.cargo, and the ~/.zshrc line
```

If `cargo install wasm-pack` was run but `rustup self uninstall` is skipped, just delete the binary:

```sh
rm ~/.cargo/bin/wasm-pack
```

## Notes

- The official rustup installer is `https://sh.rustup.rs`, served by the Rust project. The `curl | sh` form is the documented install path on rust-lang.org.
- Homebrew has a `rustup` formula, but it is a thin wrapper — under the hood it still uses rustup's own toolchain mechanism. Using the official installer directly avoids one layer of indirection and matches every Rust tutorial on the internet.
- `~/.cargo/bin` is the install destination for any binary you `cargo install`. It is on `PATH` after sourcing `~/.cargo/env`.
