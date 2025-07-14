use crate::cpac;
use crate::pac::interrupt;
use crate::pac::NVIC;
use cpac::{modify_field, read_field};
use defmt::info;

static mut SW1: bool = false;
static mut SW3: bool = false;

pub fn read_sw1() -> bool {
    unsafe { SW1 }
}

pub fn read_sw3() -> bool {
    unsafe { SW3 }
}

pub fn init_switches() {
    let rcc = cpac::rcc::RCC_TypeDef::new_static_ref();
    modify_field(&mut rcc.IOPENR, cpac::rcc::IOPENR_IOPBEN_Msk, 1);
    modify_field(&mut rcc.APB2ENR, cpac::rcc::APB2ENR_SYSCFGEN_Msk, 1);

    {
        use cpac::gpio_b::*;
        let pb = GPIO_TypeDef::new_static_ref();

        // SW1
        modify_field(&mut pb.MODER, MODER_MODE5, 0b00); // set to input
        modify_field(&mut pb.PUPDR, PUPDR_PUPD5, 0b00); // set no pull-up, no pull-down

        // SW3
        modify_field(&mut pb.MODER, MODER_MODE6, 0b00); // set to input
        modify_field(&mut pb.PUPDR, PUPDR_PUPD6, 0b00); // set no pull-up, no pull-down
    }
    // connect PB5 to EXTI5
    {
        use cpac::syscfg::*;
        let syscfg = SYSCFG_TypeDef::new_static_ref();
        modify_field(&mut syscfg.EXTICR[1], EXTICR2_EXTI5_Msk, 0b0001);
    }

    // connect PB6 to EXTI6
    {
        use cpac::syscfg::*;
        let syscfg = SYSCFG_TypeDef::new_static_ref();
        modify_field(&mut syscfg.EXTICR[1], EXTICR2_EXTI6_Msk, 0b0001);
    }

    {
        use cpac::exti::*;
        let exti = EXTI_TypeDef::new_static_ref();
        modify_field(&mut exti.RTSR, RTSR_RT5_Msk, 1);
        modify_field(&mut exti.FTSR, FTSR_FT5_Msk, 1);
        modify_field(&mut exti.IMR, IMR_IM5_Msk, 1);

        modify_field(&mut exti.RTSR, RTSR_RT6_Msk, 1);
        modify_field(&mut exti.FTSR, FTSR_FT6_Msk, 1);
        modify_field(&mut exti.IMR, IMR_IM6_Msk, 1);
    }

    unsafe {
        NVIC::unmask(interrupt::EXTI4_15);
    };
}
#[interrupt]
fn EXTI4_15() {
    exti4_15_handler();
}

#[inline(always)]
fn exti4_15_handler() {
    critical_section::with(|_cs| {
        info!("exti4_15 interrupt");
        let exti = cpac::exti::EXTI_TypeDef::new_static_ref();
        let gpio_b = cpac::gpio_b::GPIO_TypeDef::new_static_ref();

        if read_field(&exti.PR, cpac::exti::PR_PIF5_Msk) == 1 {
            // clear pending bit, PR reg is rc_w1
            modify_field(&mut exti.PR, cpac::exti::PR_PIF5_Msk, 1);

            unsafe { SW1 = read_field(&gpio_b.IDR, cpac::gpio_b::IDR_ID5_Msk) == 1 }
        }

        if read_field(&exti.PR, cpac::exti::PR_PIF6_Msk) == 1 {
            // clear pending bit, PR reg is rc_w1
            modify_field(&mut exti.PR, cpac::exti::PR_PIF6_Msk, 1);

            unsafe { SW3 = read_field(&gpio_b.IDR, cpac::gpio_b::IDR_ID6_Msk) == 1 }
        }
    })
}
