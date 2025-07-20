#![no_std]
#![no_main]

use embedded_hal::delay::DelayNs;
use panic_halt as _;

use cortex_m_rt::entry;
use defmt::info;

use bsp;
use stm32l0_cpac::{self as cpac, modify_field, read_field};

use stm32_usbd;
use usb_device::prelude::*;
use usbd_serial;

struct USBPeripheral;

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

#[entry]
fn main() -> ! {
    info!("Start");
    bsp::init();

    let rtc = cpac::rtc::RTC_TypeDef::new_static_ref();

    let usb_bus = stm32_usbd::UsbBus::new(USBPeripheral {});

    let mut serial = usbd_serial::SerialPort::new(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x6969, 0x6969))
        .strings(&[StringDescriptors::new(LangID::EN).product("Serial port")])
        .expect("Failed to set strings")
        .device_class(usbd_serial::USB_CLASS_CDC)
        .build();

    let mut usb_buffer = [0u8; 128];
    let mut bld = bsp::BusyLoopDelayNs {};

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

        // only handle USB events if sw1 is pressed
        while bsp::switches::read_sw1() {
            if usb_dev.poll(&mut [&mut serial]) {
                match serial.read(&mut usb_buffer) {
                    Ok(n_bytes) => {
                        info!("{} bytes read from the usb", n_bytes);
                    }
                    Err(e) => {
                        defmt::error!("Could not read from the USB: {}", e);
                    }
                }
            }
            bld.delay_ms(1);
        }
        bsp::enter_sleep();
    }
}
