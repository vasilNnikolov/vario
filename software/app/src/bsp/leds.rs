use crate::cpac;
use cpac::modify_field;

pub fn init_leds() {
    let rcc = cpac::rcc::RCC_TypeDef::new_static_ref();
    modify_field(&mut rcc.IOPENR, cpac::rcc::IOPENR_IOPBEN_Msk, 1);

    {
        use cpac::gpio_b::*;
        let gpio_b = GPIO_TypeDef::new_static_ref();
        modify_field(&mut gpio_b.MODER, MODER_MODE12, 0b01); // set output mode
        modify_field(&mut gpio_b.OTYPER, OTYPER_OT_12, 0b0); // set output type to push-pull
        modify_field(&mut gpio_b.PUPDR, PUPDR_PUPD12, 0b00); // set no pull-up & no pull-down

        modify_field(&mut gpio_b.MODER, MODER_MODE13, 0b01); // set output mode
        modify_field(&mut gpio_b.OTYPER, OTYPER_OT_13, 0b0); // set output type to push-pull
        modify_field(&mut gpio_b.PUPDR, PUPDR_PUPD13, 0b00); // set no pull-up & no pull-down

        modify_field(&mut gpio_b.MODER, MODER_MODE14, 0b01); // set output mode
        modify_field(&mut gpio_b.OTYPER, OTYPER_OT_14, 0b0); // set output type to push-pull
        modify_field(&mut gpio_b.PUPDR, PUPDR_PUPD14, 0b00); // set no pull-up & no pull-down
    }

    let mut bld = crate::BusyLoopDelayNs {};
    use embedded_hal::delay::DelayNs;

    for i in 0..3 * 8 {
        set_led(LED::LED1, ((i >> 0) & 1) == 1);
        set_led(LED::LED2, ((i >> 1) & 1) == 1);
        set_led(LED::LED3, ((i >> 2) & 1) == 1);
        bld.delay_ms(30);
    }
}

/// enum representing the LEDs on the board
pub enum LED {
    /// connected to PB12
    LED1,
    /// connected to PB13
    LED2,
    /// connected to PB14
    LED3,
}

pub fn set_led(led: LED, is_on: bool) {
    use cpac::gpio_b::*;
    let gpio_b = GPIO_TypeDef::new_static_ref();
    match led {
        LED::LED1 => {
            modify_field(
                &mut gpio_b.BSRR,
                if is_on { BSRR_BS_12 } else { BSRR_BR_12 },
                1,
            );
        }
        LED::LED2 => {
            modify_field(
                &mut gpio_b.BSRR,
                if is_on { BSRR_BS_13 } else { BSRR_BR_13 },
                1,
            );
        }
        LED::LED3 => {
            modify_field(
                &mut gpio_b.BSRR,
                if is_on { BSRR_BS_14 } else { BSRR_BR_14 },
                1,
            );
        }
    }
}
