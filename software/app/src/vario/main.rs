#![no_std]
#![no_main]

use embedded_hal::delay::DelayNs;
use panic_halt as _;
use usb_device::prelude::*;
use usbd_serial;

use cortex_m_rt::entry;
use defmt::info;

use bsp::cpac;

pub enum State {
    /// board is in STOP low power mode
    Off,
    /// board is in normal (not low power) mode
    On(OnState),
}

pub enum OnState {
    Normal,
    UsbTransfer,
}

#[entry]
fn main() -> ! {
    let mut s: State = State::On(OnState::Normal);
    info!("Start");
    bsp::init();

    let rtc = cpac::rtc::RTC_TypeDef::new_static_ref();

    let usb_bus = stm32_usbd::UsbBus::new(bsp::usb::USBPeripheral {});

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
