#![no_std]
#![no_main]

use embedded_hal::delay::DelayNs;
use panic_halt as _;

use cortex_m_rt::entry;
use defmt::info;
use defmt_rtt as _;
use stm32l0 as _; // do not remove, neede for compilation

use cpac::volatile_register;
use stm32l0_cpac as cpac;

struct BusyLoopDelayNs;

impl embedded_hal::delay::DelayNs for BusyLoopDelayNs {
    fn delay_ns(&mut self, ns: u32) {
        const CPU_FREQ: u32 = 2_000_000;
        let d_cycles = ns as u64 * CPU_FREQ as u64 / 1_000_000_000 as u64;
        cortex_m::asm::delay(d_cycles as u32);
    }
}

// pub unsafe fn set_val<T: Copy>(reg: &mut volatile_register::RW<T>, val: T, pos: u32, msk: u32) {
//     reg.modify(|mut x| {
//         x |= ()
//     });
// }

#[entry]
fn main() -> ! {
    info!("Start");
    let rcc = cpac::rcc::RCC_TypeDef::new_static_ref();

    let rcc = cpac::rcc::RCC_TypeDef::new_static_ref();
    unsafe {
        rcc.IOPENR.modify(|mut x| {
            x |= (1 << cpac::rcc::RCC_IOPENR_IOPBEN_Pos) & cpac::rcc::RCC_IOPENR_IOPBEN_Msk;
            x
        })
    }
    loop {}
}
