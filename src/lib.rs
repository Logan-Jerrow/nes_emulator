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
    clippy::missing_errors_doc
)]

mod addressing_mode;
pub mod cpu;
mod opcode;

/*
    Central Processing Unit (CPU)
    Picture Processing Unit (PPU)
    Random Access Memory (RAM) - 2 KiB (2048 bytes)
    Cartridges
    Gamepads
*/

// pub mod acl; // Arithmetic Logic Unit
// pub mod architecture;
// pub mod memory;
// pub mod apu; // Audio Processing Unit (APU)
// pub mod bus;
// pub mod gamepad;
// pub mod ppu;
// pub mod rom;
