//! Board support crate for the {manufacturer} {board}
//!
//! See https://example.com/path/to/page
//!
//! User Manual: https://example.com/path/to/manual.pdf
//!
#![no_std]

/// The Hardware Abstraction Layer for our SoC
pub extern crate nrf9160_hal as hal;

use hal::gpio::{p0, Floating, Input};

/// Exports traits that are usually needed when using this crate
pub mod prelude {
    pub use hal::prelude::*;
}

/// The Peripheral Access Crate for our SoC
pub use hal::pac;

/// Provides access to all features of the {manufacturer} {board}
///
/// NB: The following peripherals are not yet supported:
///
/// * UART
/// * LEDs
/// * Buttons
/// * Buzzers
/// * SPI
/// * I²C
#[allow(non_snake_case)]
pub struct Board {
    /// The {cpu} pins which are not otherwise occupied on the {manufacturer} {board}
    pub pins: Pins,
}

/// The {cpu} pins that are available on the {manufacturer} {board}
#[allow(non_snake_case)]
pub struct Pins {
    /// Pin description here
    pub P0_00: p0::P0_00<Input<Floating>>,
    /// Pin description here
    pub P0_01: p0::P0_01<Input<Floating>>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
