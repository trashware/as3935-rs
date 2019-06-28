use crate::interface::Interface;
use crate::interface::{calculate_bitshift, Result};
use rppal::i2c::I2c;

pub const DEFAULT_I2C_ADDRESS: u8 = 0x03;

pub struct I2cAddress(pub(crate) u8);

impl I2cAddress {
    pub fn new(address: u8) -> Self {
        if address > 127 {
            panic!("invalid I2C address")
        }

        Self(address)
    }

    pub fn default() -> Self {
        Self::new(DEFAULT_I2C_ADDRESS)
    }
}

pub(crate) struct I2cInterface {
    i2c: I2c,
}

impl I2cInterface {
    pub(crate) fn new(mut i2c: I2c, i2c_address: I2cAddress) -> Result<Self> {
        i2c.set_slave_address(i2c_address.into())?;

        Ok(Self { i2c })
    }
}

impl Interface for I2cInterface {
    fn read(&mut self, register: Box<dyn crate::device::registers::Register>) -> Result<u8> {
        let mut data: [u8; 1] = [0];

        self.i2c.write_read(&[register.address()], &mut data)?;

        let value = (data[0] & register.mask()) >> calculate_bitshift(register.mask());
        debug!("read {} = {:#b}", register.name(), value);

        Ok(value)
    }

    fn write(
        &mut self,
        register: Box<dyn crate::device::registers::Register>,
        payload: u8,
    ) -> Result<()> {
        debug!("setting {} = {:#b}", register.name(), payload);

        let bitshift = calculate_bitshift(register.mask());
        assert!(payload <= (register.mask() >> bitshift));

        let mut current_data: [u8; 1] = [0];
        self.i2c
            .write_read(&[register.address()], &mut current_data)?;

        self.i2c.write(&[
            register.address(),
            (current_data[0] ^ register.mask()) | (payload << bitshift),
        ])?;

        Ok(())
    }
}
