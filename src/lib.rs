#![warn(
    clippy::all,
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::style,
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used,
    clippy::expect_used
)]
#![allow(
    dead_code,
    unused,
    clippy::missing_panics_doc,
    clippy::missing_errors_doc
)]

pub mod acl; // Arithmetic Logic Unit
pub mod architecture;
pub mod memory;

/*
    Central Processing Unit (CPU)
    Picture Processing Unit (PPU)
    Random Access Memory (RAM) - 2 KiB (2048 bytes)
    Cartridges
    Gamepads
*/

pub mod apu; // Audio Processing Unit (APU)
pub mod bus;
pub mod cpu;
pub mod gamepad;
mod opcode;
pub mod ppu;
pub mod rom;
