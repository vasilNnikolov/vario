#![no_std]
#![no_main]

use embedded_hal::delay::DelayNs;
use panic_halt as _;

use bme280_bindings_rs;
use bme280_bindings_rs::bindings as bme_b;
use cortex_m::asm;
use cortex_m_rt::entry;
use defmt::info;
use defmt_rtt as _;
use stm32l0::stm32l0x2 as pac;

fn log_resets(rcc: &pac::RCC) {
    let low_power_reset = rcc.csr.read().porrstf().bit_is_set();
    info!("POR true/false: {}", low_power_reset);
}

struct BusyLoopDelayNs;

impl embedded_hal::delay::DelayNs for BusyLoopDelayNs {
    fn delay_ns(&mut self, ns: u32) {
        for _ in 0..(ns * 8) {
            asm::nop();
        }
    }
}

#[entry]
fn main() -> ! {
    info!("Start");
    let _core_p = cortex_m::Peripherals::take().unwrap();
    let p = pac::Peripherals::take().unwrap();

    log_resets(&p.RCC);
    p.RCC.iopenr.modify(|_, w| w.iopaen().bit(true));
    p.GPIOA.moder.modify(|_, w| w.mode8().output());
    p.GPIOA.otyper.modify(|_, w| w.ot8().push_pull());
    p.RCC.ahbenr.modify(|_, w| w.mifen().set_bit());

    let mut bld = BusyLoopDelayNs;
    let mut i = 0;
    // let mut i2c_dev = ();
    // // let calib_data = bme_b::bme280_calib_data::default();
    // let void_ptr = &mut i2c_dev as *mut _ as *mut core::ffi::c_void;
    // let _ = bme_b::bme280_dev {
    //     chip_id: 0,
    //     intf: bme_b::bme280_intf_BME280_I2C_INTF,
    //     intf_ptr: void_ptr,
    //     intf_rslt: 0,
    //     read: None,
    //     write: None,
    //     delay_us: None,
    //     calib_data,
    // };
    let _ = bme_b::BME280_12_BIT_SHIFT;
    loop {
        info!("Counter: {}", i);
        if i & 1 == 0 {
            // turn on
            p.GPIOA.bsrr.write(|w| w.bs8().set_bit());
        } else {
            // turn off
            p.GPIOA.bsrr.write(|w| w.br8().set_bit());
        }

        i += 1;
        bld.delay_ms(1000);
    }
}
