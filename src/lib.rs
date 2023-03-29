#![warn(
    clippy::cargo,
    clippy::complexity,
    clippy::correctness,
    clippy::nursery,
    clippy::pedantic,
    clippy::perf,
    clippy::style,
    clippy::suspicious
)]
// clippy::restriction
#![warn(clippy::unwrap_used, clippy::expect_used)]
#![allow(
    dead_code,
    unused,
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::multiple_crate_versions
)]

mod addressing_mode;
mod bus;
pub mod cpu;
mod opcode;

/*
    Central Processing Unit (CPU)
    Picture Processing Unit (PPU)
    Random Access Memory (RAM) - 2 KiB (2048 bytes)
    Cartridges
    Gamepads
*/
