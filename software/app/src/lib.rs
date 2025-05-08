#![no_std]

pub use stm32l0::stm32l0x2 as pac;

// pub struct Pin {
//     /// port (A,B,..) is 4 MSB, pin number is 4 LSB
//     /// PA13 = 0b00001101=13, PB5=0b00010101=21
//     port_num: u8,
// }

// impl Pin {
//     pub fn get_port(&self) -> u8 {
//         self.port_num >> 4
//     }
//     pub fn get_pin(&self) -> u8 {
//         self.port_num & 0xf
//     }

// }

pub mod memory_spi {
    use super::*;
    pub fn init_spi(p: &mut pac::Peripherals) {}
}

pub mod bme_i2c {
    use super::*;
    pub fn init_i2c(p: &mut pac::Peripherals) {}
}
