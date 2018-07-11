#![no_std]

extern crate embedded_hal as hal;

use hal::blocking::i2c;

pub struct Pcf8574<I2C> {
    i2c: I2C,
    address: u8,
}

impl <I2C, E> Pcf8574<I2C>
where
     I2C: i2c::Write<Error = E>,
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

impl <I2C, E> Pcf8574<I2C>
where
    I2C: i2c::Read<Error = E> + i2c::Write<Error = E>,
{
    pub fn get(&mut self, mask: u8) -> Result<u8, E> {
        // Enable pins for reading
        self.i2c.write(self.address, &[mask])?;

        let mut buf = [0u8; 1];
        self.i2c.read(self.address, &mut buf)?;
        Ok(buf[0])
    }
}
