#![no_std]
#![no_main]

use embedded_hal::delay::DelayNs;
use panic_halt as _;

use bsp::pac;
use cortex_m_rt::entry;
use defmt::info;
use defmt_rtt as _;
use stm32l0 as _; // do not remove, needed for compilation and filling in interrupts

use cpac::modify_reg;
use stm32l0_cpac as cpac;

struct BusyLoopDelayNs;

impl embedded_hal::delay::DelayNs for BusyLoopDelayNs {
    fn delay_ns(&mut self, ns: u32) {
        const CPU_FREQ: u32 = 2_000_000;
        let d_cycles = ns as u64 * CPU_FREQ as u64 / 1_000_000_000 as u64;
        cortex_m::asm::delay(d_cycles as u32);
    }
}

#[entry]
fn main() -> ! {
    info!("Start");
    let p = pac::Peripherals::take().unwrap();
    p.RCC.iopenr.modify(|_, w| w.iopben().set_bit());
    // let rcc = cpac::rcc::RCC_TypeDef::new_static_ref();
    // modify_reg(
    //     &mut rcc.IOPENR,
    //     cpac::rcc::IOPENR_IOPBEN_Msk,
    //     cpac::rcc::IOPENR_IOPBEN_Pos,
    //     1,
    // );

    let gpiob = &p.GPIOB;
    gpiob.moder.modify(|_, w| w.mode12().output());
    gpiob.otyper.modify(|_, w| w.ot12().clear_bit());
    gpiob.pupdr.modify(|_, w| w.pupd12().floating());

    // let gpio_b = cpac::gpio_b::GPIO_TypeDef::new_static_ref();
    // // set output mode
    // modify_reg(
    //     &mut gpio_b.MODER,
    //     cpac::gpio_b::MODER_MODE12,
    //     cpac::gpio_b::MODER_MODE12_Pos,
    //     0b11,
    // );
    // // set output type to push-pull
    // modify_reg(&mut gpio_b.OTYPER, cpac::gpio_b::OTYPER_OT_12, 12, 0b0);
    // // set no pull-up & no pull-down
    // modify_reg(
    //     &mut gpio_b.PUPDR,
    //     cpac::gpio_b::PUPDR_PUPD12,
    //     cpac::gpio_b::PUPDR_PUPD12_Pos,
    //     0b00,
    // );

    let mut bld = BusyLoopDelayNs;
    let mut i = 0;
    loop {
        info!("Counter: {}", i);
        // turn PB12 on
        p.GPIOB.bsrr.write(|w| w.br12().set_bit());
        // modify_reg(&mut gpio_b.BSRR, cpac::gpio_b::BSRR_BS_12, 12, 0b1);
        bld.delay_ms(100);

        // turn PB12 off
        p.GPIOB.bsrr.write(|w| w.bs12().set_bit());
        // modify_reg(&mut gpio_b.BSRR, cpac::gpio_b::BSRR_BR_12, 28, 0b1);
        bld.delay_ms(100);
        i += 1;
    }
}
