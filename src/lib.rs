//! Sensors: Data-driven sensor models for simulation.
//!
//! # Overview
//! `control-rs-sensors` provides sensor models identified from recorded
//! hardware data, for use as measurement sources in `control-rs` simulations.
//!
//! # Quality Gate
//! `#![no_std]` compatible. Gated by `control-rs-ci` (`.cargo/gate.toml`).

#![cfg_attr(not(feature = "std"), no_std)]
