#![no_std]
#![no_main]

//! cnachen's [Rust](https://www.rust-lang.org) template for `stm32f411`
//!
//! A crate provides a basic `stm32f411` configuration which is available out of box.
//!
//! # Quick Start
//!
//! - Cargo.toml
//! ```toml
//! [dependencies.stm32-hal2]
//! version = "1.3"
//! features = ["f411"] // Your MCU model here.
//! ```
//!
//! - memory.x
//! ```text
//! MEMORY
//! {
//!     FLASH : ORIGIN = 0x08000000, LENGTH = 512K
//!     RAM : ORIGIN = 0x20000000, LENGTH = 128K
//! }
//! ```
//!
//! - src/inf.rs
//! ```rust
//! #[entry]
//! fn main -> ! {
//!     asm::nop();
//!     // Your code here.
//!     loop {}
//! }
//! ```
//!
//! # Useful Resources
//!
//! [STM32F411CE](https://www.st.com/en/microcontrollers-microprocessors/stm32f411ce.html)
//!
//! [Writing embedded firmware using Rust](https://www.anyleaf.org/blog/writing-embedded-firmware-using-rust)
//!
//! # Todo
//!
//! - Make LED blinky in real life.
//!

use panic_halt as _;

#[allow(unused_imports)]
use stm32_hal2 as hal;

use cortex_m::asm;
use cortex_m_rt::entry;

/// Entry point of user code.
#[doc(hidden)]
#[entry]
fn main() -> ! {
    fork();
    execv();
    asm::nop();
    loop {}
}

/// Fork a new process.
fn fork() {
    let _x = 42;
}

/// Execute a program.
fn execv() {
    let _x = 42;
}