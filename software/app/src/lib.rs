#![no_std]

use cortex_m_rt::exception;
use defmt::info;
pub use stm32l0::stm32l0x2 as pac;

// pub mod rcc_setup {
//     use super::*;

//     pub fn start_HSE(p: &mut pac::Peripherals, dp: &mut cortex_m::Peripherals) {
//         unsafe {
//             pac::NVIC::unmask(pac::interrupt::RCC);

//             let nvic = &mut *(pac::NVIC::PTR as *mut pac::NVIC);
//             nvic.set_priority(pac::interrupt::RCC, 0);
//         }
//         p.RCC.cier.read()
//     }

//     pub fn rcc_init(p: &mut pac::Peripherals, dp: &mut cortex_m::Peripherals) {
//         // start HSE
//         // p.RCC.cr.modify(|_,w| w.pllon().clear_bit().rtcpre());
//     }
// }

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

#[exception]
unsafe fn DefaultHandler(irq_num: i16) {
    info!(
        "The interrupt with number {} went to DefaultHandler",
        irq_num
    );
}
