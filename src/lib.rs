#![no_std]

mod reg;
pub use crate::reg::*;

mod non_blocking;
pub use non_blocking::*;

use core::fmt::Debug;

pub use accelerometer::{vector::F32x3, Accelerometer};

#[derive(Debug)]
pub enum Error<SpiError, PinError> {
    /// SPI communication error
    Spi(SpiError),
    /// CS output pin error
    Pin(PinError),
    InvalidWhoAmI(u8),
}

impl<SpiError, PinError> From<SpiError> for Error<SpiError, PinError> {
    fn from(err: SpiError) -> Self {
        Self::Spi(err)
    }
}

pub struct Lis2dh12<SPI, CS> {
    spi: SPI,
    cs: CS,
    scale: FullScaleSelection,
}

impl<SPI, CS> Lis2dh12<SPI, CS> {
    pub fn new(spi: SPI, cs: CS) -> Self {
        Self {
            spi,
            cs,
            scale: FullScaleSelection::PlusMinus2G,
        }
    }

    // destroy the instance and return the spi bus and its cs pin
    pub fn destroy(self) -> (SPI, CS) {
        (self.spi, self.cs)
    }
}
