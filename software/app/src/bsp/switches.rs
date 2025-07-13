use crate::cpac;
use crate::pac::interrupt;
use crate::pac::NVIC;
use cpac::modify_field;

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
