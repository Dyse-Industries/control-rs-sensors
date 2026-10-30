# `control-rs-sensors`

> **Dyse-Industries System Ecosystem — Standalone Crate**

| Metadata | Details |
|---|---|
| **Crate Name** | `control-rs-sensors` |
| **Version** | `0.1.0` |
| **License** | `MIT OR Apache-2.0` |
| **`no_std`** | Fully Supported (`default = ["std"]`) |
| **Repository** | [https://github.com/Dyse-Industries/control-rs-sensors](https://github.com/Dyse-Industries/control-rs-sensors) |
| **Quality Gate** | `control-rs-ci` (`.cargo/gate.toml`) |

---

## 1. Overview

Data-driven sensor models for simulation. Each model is identified from recorded hardware data and reproduces the measured output of a physical sensor (noise, bias, scale, latency and saturation) as a measurement source for `control-rs` simulations.

---

## 2. Quality Gate & Ecosystem Integration

Depends on [`control-rs`](https://github.com/Dyse-Industries/control-rs) by git revision (`Cargo.toml`). Quality gates run through `control-rs-ci`, built from the same revision, with gates declared in [`.cargo/gate.toml`](.cargo/gate.toml):

```bash
# Install the gate runner at the pinned control-rs revision
cargo install --locked --git https://github.com/Dyse-Industries/control-rs \
    --rev "$(grep -oP 'control-rs\?rev=\K[0-9a-f]{40}' Cargo.lock | head -1)" \
    control-rs-ci --bin control-rs-ci

control-rs-ci              # all gates; artifacts in target/ci-artifacts
control-rs-ci --list       # registered gates
control-rs-ci --only fmt,clippy
```

---

## 3. License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT) at your option.
