#![no_std]
#![no_main]

use cortex_m::asm::wfi;
use embedded_hal::delay::DelayNs;
use panic_halt as _;
use usb_device::prelude::*;
use usbd_serial;

use cortex_m_rt::entry;
use defmt::{info, warn};

use bsp::cpac::{self, read_field};

#[derive(Debug, defmt::Format)]
pub enum State {
    /// board is in STOP low power mode
    StandbyMode,
    /// board is in normal (not low power) mode
    RunMode(RunMode),
}

#[derive(Debug, defmt::Format)]
pub enum RunMode {
    Normal,
    UsbTransfer,
    /// when the systick has ticked past the argument, the TransitionToStop ends, and the board goes into StopMode
    TransitionToStandby(u64),
    /// when the systick has ticked past the argument, the TransitionToStart ends, and the board goes into NormalMode
    TransitionToStart(u64),
}

// #[entry]
fn main2() -> ! {
    info!("Start");
    bsp::init();
    loop {
        let systick = bsp::systick::get_systic_ticks();
        info!("systick: {}", systick);

        if systick > 5 {
            bsp::configure_standby_mode();
        }
        wfi();
    }
}

#[entry]
fn main() -> ! {
    let mut s: State = State::RunMode(RunMode::Normal);
    info!("Start");
    bsp::init();

    let rtc = cpac::rtc::RTC_TypeDef::new_static_ref();

    // let usb_bus = stm32_usbd::UsbBus::new(bsp::usb::USBPeripheral {});

    // let mut serial = usbd_serial::SerialPort::new(&usb_bus);

    // let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x6969, 0x6969))
    //     .strings(&[StringDescriptors::new(LangID::EN).product("Serial port")])
    //     .expect("Failed to set strings")
    //     .device_class(usbd_serial::USB_CLASS_CDC)
    //     .build();

    // let mut usb_buffer = [0u8; 128];
    // let mut bld = bsp::BusyLoopDelayNs {};

    loop {
        info!("State: {}; Uptime {}s", s, bsp::systick::get_systic_ticks());

        let date_reg = cpac::read_field(&mut rtc.DR, u32::MAX);
        let time_reg = cpac::read_field(&mut rtc.TR, u32::MAX);
        info!("date reg: {=u32:x}", date_reg);
        info!("time reg: {=u32:x}", time_reg);
        let sw1 = bsp::switches::read_sw1();
        bsp::leds::set_led(bsp::leds::LED::LED1, sw1);

        let sw2 = bsp::switches::read_sw2();
        bsp::leds::set_led(bsp::leds::LED::LED2, sw2);

        let sw3 = bsp::switches::read_sw3();
        bsp::leds::set_led(bsp::leds::LED::LED3, sw3);

        let pwr = cpac::pwr::PWR_TypeDef::new_static_ref();
        let standby_flag = read_field(&pwr.CSR, cpac::pwr::CSR_SBF_Msk);
        info!("standby flag: {}", standby_flag);

        match s {
            State::RunMode(ref rm) => match *rm {
                RunMode::Normal => {
                    if sw2 {
                        warn!("going to RunMode, TransitionToStop");
                        s = State::RunMode(RunMode::TransitionToStandby(
                            bsp::systick::get_systic_ticks() + 3,
                        ));
                    } else if sw1 {
                        warn!("going to RunMode, UsbTransfer");
                        s = State::RunMode(RunMode::UsbTransfer);
                    }
                }
                RunMode::UsbTransfer => {
                    if sw2 {
                        warn!("going to RunMode, TransitionToStop");
                        s = State::RunMode(RunMode::TransitionToStandby(
                            bsp::systick::get_systic_ticks() + 3,
                        ));
                    } else if !sw1 {
                        warn!("going into RunMode, Normal");
                        s = State::RunMode(RunMode::Normal);
                    }

                    // // only handle USB events if sw1 is pressed
                    // while bsp::switches::read_sw1() {
                    //     if usb_dev.poll(&mut [&mut serial]) {
                    //         match serial.read(&mut usb_buffer) {
                    //             Ok(n_bytes) => {
                    //                 warn!("{} bytes read from the usb", n_bytes);
                    //             }
                    //             Err(e) => {
                    //                 defmt::error!("Could not read from the USB: {}", e);
                    //             }
                    //         }
                    //     }
                    //     bld.delay_ms(1);
                    // }
                }
                RunMode::TransitionToStandby(i) => {
                    if bsp::systick::get_systic_ticks() > i {
                        warn!("going to StopMode");
                        s = State::StandbyMode;
                    } else if !sw2 {
                        warn!("going to RunMode, Normal");
                        s = State::RunMode(RunMode::Normal);
                    }
                }
                RunMode::TransitionToStart(i) => {
                    if bsp::systick::get_systic_ticks() > i {
                        warn!("going to RunMode, Normal");
                        s = State::RunMode(RunMode::Normal);
                    } else if !sw2 {
                        warn!("going to StopMode");
                        s = State::StandbyMode;
                    }
                }
            },
            State::StandbyMode => {
                bsp::leds::set_led(bsp::leds::LED::LED2, false);
                // let mut bld = bsp::BusyLoopDelayNs {};
                // for _ in 0..5_000 {
                //     bld.delay_ms(1);
                // }
                bsp::configure_standby_mode();
                wfi();
                // // exiting sends us at the beginning of the program
                // bsp::configure_standby_mode();

                // // // going into Stop mode implicitly resets the state back to the initial state, which must be TransitionToStart
                // // s = State::RunMode(RunMode::TransitionToStart(
                // //     bsp::systick::get_systic_ticks() + 5,
                // // ));
                // cortex_m::asm::wfi();
            }
        }

        // wfi();
        bsp::enter_sleep();
    }
}
