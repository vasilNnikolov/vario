//! This example test the RP Pico on board LED.
//!
//! It does not work with the RP Pico W board. See wifi_blinky.rs.

#![no_std]
#![no_main]

use bme280;
use bme280::{Configuration, IIRFilter, Oversampling};
use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio;
use embassy_rp::i2c::{self, Config};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::{Delay, Duration, Instant, Timer};
use embedded_hal::delay::DelayNs;
use gpio::{Level, Output};
use {defmt_rtt as _, panic_probe as _};

type LedType = Mutex<ThreadModeRawMutex, Option<Output<'static>>>;
static LED: LedType = Mutex::new(None);
const G: f32 = 9.81; // m.s^-2
const MU: f32 = 29e-3; // kg.mol^-1
const R: f32 = 8.314; // J.mol^-1
const DT: Duration = Duration::from_millis(20);

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
    let i2c = i2c::I2c::new_blocking(p.I2C1, scl, sda, Config::default());

    let mut ps = bme280::i2c::BME280::new_primary(i2c);
    let mut d = Delay {};

    let bme_config = Configuration::default()
        .with_pressure_oversampling(Oversampling::Oversampling16X)
        .with_temperature_oversampling(Oversampling::Oversampling1X)
        .with_iir_filter(IIRFilter::Coefficient16);

    match ps.init_with_config(&mut d, bme_config) {
        Ok(_) => {
            info!("BME init successful");
        }
        Err(_) => {
            error!("BME init failed");
        }
    }
    // let _ = ps.set_normal_mode(&mut d);
    let _ = ps.common.set_normal_mode(&mut d);
    d.delay_ms(1000);
    let mut last_p: Option<f32> = None;
    let mut last_t: Option<Instant> = None;
    let mut h: f32 = 0.;

    loop {
        match ps.measure(&mut d) {
            Ok(data) => {
                let p = data.pressure;
                let t = data.temperature;
                let now = Instant::now();
                // info!("{} Pa, {} C", p, t);

                if let (Some(last_p), Some(last_t)) = (last_p, last_t) {
                    let dpdt = (p - last_p) / ((now - last_t).as_micros() as f32 / 1_000_000.0);

                    let v = -(R * (t + 273.15)) / (G * p * MU) * dpdt;
                    h += v * (DT.as_millis() as f32 / 1000.);

                    info!(
                        "Vertical speed {} cm/s, height {} cm",
                        100. * ((1000.0 * v) as i32) as f32 / 1000.0,
                        100. * ((1000.0 * h) as i32) as f32 / 1000.0,
                    );
                }
                last_p = Some(p);
                last_t = Some(now);
            }
            Err(_) => {
                warn!("Could not measure from BME280");
                d.delay_ms(100);
            }
        }
        Timer::after(DT).await;
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
