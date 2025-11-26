# Overview

This repository contains a repro for https://github.com/rust-lang/cargo/issues/16297.

It requires a workspace with:
- A binary crate that is linked against [bevy_dylib](https://docs.rs/bevy_dylib/latest/bevy_dylib/) using the `bevy/dynamic_linking` feature
- A proc-macro crate that contains a non-default executable (placed under `src/bin/`), and that depends on a the aforementioned binary crate linked

This triggers a `artifact-dir was not locked` error upon running `cargo check --workspace` with rust nightly >= 2025-11-21.

See [cargo-check-workspace-trace.log](cargo-check-workspace-trace.log).
