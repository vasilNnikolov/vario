#![no_std]
#![no_main]
use panic_halt as _;

use cortex_m_rt::entry;
use defmt::info;
use defmt_rtt as _;
use stm32l0::stm32l0x2 as pac;
use stm32l0::stm32l0x2::interrupt;

/// TIM6 is configured to run at 16MHz, and overflow at its max, 0xffff
/// Therefore, it will overflow approx. every 4ms
// pub mod tim6_delay {
//     use super::*;
//     pub const TIM6_FREQ: u32 = 16_000_000;

//     static mut TIM6_OVERFLOWS: u64 = 0;

//     pub fn init_tim6(p: &mut pac::Peripherals) {
//         p.RCC.icscr

//         p.TIM6.dier.modify(|_, w| w.uie().set_bit());
//         p.TIM6.psc.write(|w| unsafe { w.bits(0) });
//         p.TIM6.arr.write(|w| unsafe { w.bits(0xffff) });
//         p.TIM6.cr1.modify(|_, w| w.cen().set_bit());
//     }

//     #[derive(Default)]
//     pub struct Delay {}

//     impl embedded_hal::delay::DelayNs for Delay {
//         fn delay_ns(&mut self, ns: u32) {
//             let ticks: u32 = ((TIM6_FREQ * ns) as u64 / 1_000_000_000) as u32;
//             let n_overflows: u32 = ticks / TIM6_FREQ;
//         }
//     }
// #[interrupt]
// fn TIM6_DAC() {
//     critical_section::with(|_| unsafe {
//         pac::Peripherals::steal()
//             .TIM6
//             .sr
//             .write(|w| w.uif().clear_bit());

//         TIM6_OVERFLOWS = TIM6_OVERFLOWS.wrapping_add(1);
//     })
// }
// }

#[entry]
fn main() -> ! {
    info!("Start");
    let mut core_p = cortex_m::Peripherals::take().unwrap();
    let p = pac::Peripherals::take().unwrap();

    loop {}
}
