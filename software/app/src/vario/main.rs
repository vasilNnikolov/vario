#![no_std]
#![no_main]

use core::ops::DerefMut;

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
        bsp::enter_sleep();
        info!("Uptime {}s", bsp::systick::get_systic_ticks());

        let date_reg = cpac::read_field(&mut rtc.DR, u32::MAX);
        let time_reg = cpac::read_field(&mut rtc.TR, u32::MAX);
        info!("date reg: {=u32:x}", date_reg);
        info!("time reg: {=u32:x}", time_reg);

        let evt = critical_section::with(|cs| {
            let mut x = bsp::EVT_Q.borrow(cs).borrow_mut();
            match x.deref_mut() {
                Some(q) => q.pop_front(),
                _ => {
                    defmt::panic!("Event queue is None in the main event loop")
                }
            }
        });

        match evt {
            Some(bsp::pac::interrupt) => {}
            Some(unrecognized_idx) => {
                defmt::error!(
                    "Event with index {} went to the event loop",
                    unrecognized_idx
                )
            }
            None => {}
        };

        i += 1;
    }
}
