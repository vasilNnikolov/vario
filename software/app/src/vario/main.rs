#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;
use defmt::info;

use bsp;
use stm32l0_cpac as cpac;

#[entry]
fn main() -> ! {
    info!("Start");
    bsp::init();

    let mut i = 0;
    let rtc = cpac::rtc::RTC_TypeDef::new_static_ref();

    loop {
        info!("Uptime {}s", bsp::systick::get_systic_ticks());

        let date_reg = cpac::read_field(&mut rtc.DR, u32::MAX);
        let time_reg = cpac::read_field(&mut rtc.TR, u32::MAX);
        info!("date reg: {=u32:x}", date_reg);
        info!("time reg: {=u32:x}", time_reg);

        let sw1 = bsp::switches::read_sw1();
        bsp::leds::set_led(bsp::leds::LED::LED1, sw1);

        let sw3 = bsp::switches::read_sw3();
        bsp::leds::set_led(bsp::leds::LED::LED3, sw3);

        bsp::enter_sleep();
        i += 1;
    }
}
