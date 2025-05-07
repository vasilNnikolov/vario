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
    let mut bme_dev = bme_b::bme280_dev::default();
    //TODO set read and write functions
    bme_dev.intf = bme_b::bme280_intf_BME280_I2C_INTF;
    unsafe {
        let _retcode = bme_b::bme280_init(&mut bme_dev as *mut _);
    }

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
