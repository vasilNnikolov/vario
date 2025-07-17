#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;
use defmt::info;

use bsp;
use stm32l0_cpac as cpac;

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
        // see https://github.com/stm32-rs/stm32f0xx-hal/blob/master/src/usb.rs
    }

    fn startup_delay() {}
}

fn init_usb() {
    let usb_bus = stm32_usbd::UsbBus::new(USBPeripheral {});

    let mut serial = usbd_serial::SerialPort::new(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x6969, 0x6969))
        .strings(&[StringDescriptors::new(LangID::EN).product("Serial port")])
        .expect("Failed to set strings")
        .device_class(usbd_serial::USB_CLASS_CDC)
        .build();

    // At this point the USB peripheral is enabled and a connected host will attempt to enumerate
    // it.
    loop {
        // Must be called more often than once every 10ms to handle events and stay USB compilant,
        // or from a device-specific interrupt handler.
        if usb_dev.poll(&mut [&mut serial]) {
            // Call class-specific methods here
            // serial.read(...);
        }
    }
}

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
