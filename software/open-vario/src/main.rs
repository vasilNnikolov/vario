//! This example test the RP Pico on board LED.
//!
//! It does not work with the RP Pico W board. See wifi_blinky.rs.

#![no_std]
#![no_main]

use bme280;
use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio;
use embassy_rp::i2c::{self, Config};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::{Delay, Timer};
use gpio::{Level, Output};
use {defmt_rtt as _, panic_probe as _};

type LedType = Mutex<ThreadModeRawMutex, Option<Output<'static>>>;
static LED: LedType = Mutex::new(None);

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    // set up blinking LED
    {
        *(LED.lock().await) = Some(Output::new(p.PIN_25, Level::Low));
    }

    unwrap!(spawner.spawn(blink_led(&LED)));

    // set up BME280 I2C
    let sda = p.PIN_14;
    let scl = p.PIN_15;

    info!("set up i2c ");
    let mut i2c = i2c::I2c::new_blocking(p.I2C1, scl, sda, Config::default());

    let mut ps = bme280::i2c::BME280::new_primary(i2c);
    let mut d = Delay {};

    match ps.init(&mut d) {
        Ok(_) => {
            info!("BME init successful");
        }
        Err(_) => {
            error!("BME init failed");
        }
    }
    info!("End of program");

    loop {
        match ps.measure(&mut d) {
            Ok(data) => {
                let p = data.pressure;
                let t = data.temperature;
                info!("{} Pa, {} C", p, t);
            }
            Err(_) => {
                error!("Could not measure from BME280");
            }
        }
        Timer::after_millis(1000).await;
    }
}

#[embassy_executor::task]
async fn blink_led(led: &'static LedType) {
    loop {
        {
            let mut led_unlocked = led.lock().await;
            if let Some(pin_ref) = led_unlocked.as_mut() {
                pin_ref.set_high();
            }
            debug!("LED on")
        }
        Timer::after_millis(100).await;
        {
            let mut led_unlocked = led.lock().await;
            if let Some(pin_ref) = led_unlocked.as_mut() {
                pin_ref.set_low();
            }
            debug!("LED off")
        }
        Timer::after_millis(1000).await;
    }
}
