use defmt::info;
use stm32_usbd;

use crate::cpac;
use crate::cpac::{modify_field, read_field};

pub struct USBPeripheral;

unsafe impl stm32_usbd::UsbPeripheral for USBPeripheral {
    const REGISTERS: *const () = cpac::usb::USB_BASE as *const ();

    const DP_PULL_UP_FEATURE: bool = true;

    const EP_MEMORY: *const () = cpac::usb::EP1R as *const ();

    const EP_MEMORY_SIZE: usize = 1024;
    const EP_MEMORY_ACCESS_2X16: bool = true;

    fn enable() {
        // USB clock
        let rcc = cpac::rcc::RCC_TypeDef::new_static_ref();
        modify_field(&mut rcc.CRRCR, cpac::rcc::CRRCR_HSI48ON_Msk, 1);
        while read_field(&rcc.CRRCR, cpac::rcc::CRRCR_HSI48RDY_Msk) != 1 {
            info!("waiting for hsi48 to be ready");
        }

        modify_field(&mut rcc.CCIPR, cpac::rcc::CCIPR_HSI48SEL_Msk, 1);
        // see https://github.com/stm32-rs/stm32f0xx-hal/blob/master/src/us

        modify_field(&mut rcc.APB1ENR, cpac::rcc::APB1ENR_USBEN_Msk, 1);
        modify_field(&mut rcc.APB1RSTR, cpac::rcc::APB1RSTR_USBRST_Msk, 1);
        // the rest hopefully handled by the `std32_usbd` crate???
    }

    fn startup_delay() {
        cortex_m::asm::delay(200);
    }
}
