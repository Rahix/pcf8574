#![no_std]

extern crate embedded_hal as hal;

use hal::blocking::i2c::Write;

pub struct Pcf8574<I2C> {
    i2c: I2C,
    address: u8,
}

impl <I2C, E> Pcf8574<I2C>
where
     I2C: Write<Error = E>,
{
    pub fn new(i2c: I2C, address: u8) -> Result<Self, E> {
        Ok(Pcf8574 {
            i2c,
            address,
        })
    }

    pub fn set(&mut self, bits: u8) -> Result<(), E> {
        self.i2c.write(self.address, &[bits])?;
        Ok(())
    }
}
