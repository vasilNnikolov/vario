#![no_std]
#![no_main]

use bme280;
use bme280::{Configuration, IIRFilter, Oversampling};
use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::i2c::{self, Config};
use embassy_rp::{gpio, pwm};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::{Delay, Duration, Instant, Timer};
use embedded_hal::delay::DelayNs;
use gpio::{Level, Output};
use {defmt_rtt as _, panic_probe as _};
mod fir;

const G: f32 = 9.81; // m.s^-2
const MU: f32 = 29e-3; // kg.mol^-1
const R: f32 = 8.314; // J.mol^-1
/// the delay between each poll of the pressure sensor
const LOOP_DT: Duration = Duration::from_millis(500);
/// the duration of one beep
const BEEP_TIME: Duration = Duration::from_millis(400);
const N_FREQUENCY_INCREASES: u8 = 50;

// a type allowing to share peripherals
type Shared<T> = Mutex<ThreadModeRawMutex, T>;

static VELOCITY: Shared<f32> = Mutex::new(0.0);
/// if the height changes by that much, a beep is played out
const BEEP_INTERVAL: f32 = 0.5; // m

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    unwrap!(spawner.spawn(blink_led(Output::new(p.PIN_25, Level::Low))));

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

    ps.init_with_config(&mut d, bme_config)
        .unwrap_or_else(|_| core::panic!("could not inint BME280"));

    info!("BME init successful");

    let _ = ps.common.set_normal_mode(&mut d);
    d.delay_ms(1000);
    let mut last_p: Option<f32> = None;
    let mut last_t: Option<Instant> = None;
    let mut h: f32 = 0.;
    let mut filtered_h: f32 = 0.;

    let mut fir = fir::moving_average!(30);

    // set up pwm on pin 1, frequency ~1907 Hz
    let mut pwm_config = pwm::Config::default();
    pwm_config.top = 0xffff;
    pwm_config.compare_b = 0x7fff;
    let mut pwm = pwm::Pwm::new_output_b(p.PWM_SLICE0, p.PIN_1, pwm_config.clone());
    Timer::after(Duration::from_secs(2)).await;
    let mut last_beep_hight: f32 = 0.;

    loop {
        match ps.measure(&mut d) {
            Ok(data) => {
                let p = data.pressure;
                let t = data.temperature;
                let now = Instant::now();

                if let (None, None) = (last_p, last_t) {
                    last_p = Some(p);
                    last_t = Some(now);
                    continue;
                }

                let (last_p, last_t) = (last_p.unwrap(), last_t.unwrap());
                let dt = now - last_t;
                let dpdt = (p - last_p) / (dt.as_micros() as f32 / 1_000_000.0);

                let v = -(R * (t + 273.15)) / (G * p * MU) * dpdt;
                h += v * (dt.as_millis() as f32 / 1000.);

                fir.feed(v);
                let filtered_v = fir.output();
                {
                    *(VELOCITY.lock().await) = filtered_v;
                }
                filtered_h += filtered_v * (dt.as_millis() as f32 / 1000.);

                info!(
                    "VAR t_ms {} ms, VAR pressure {} Pa, VAR dpdt {} Pa/s, VAR veritcal_speed {} cm/s, VAR height {} cm, VAR filtered_v {} cm/s, VAR filtered_h {} cm",
                    now.as_millis(),
                    p,
                    dpdt,
                    100. * v,
                    100. * h,
                    100. * filtered_v,
                    100. * filtered_h
                );
                if filtered_h - last_beep_hight > BEEP_INTERVAL {
                    last_beep_hight = filtered_h;
                    beep(&mut pwm, true).await;
                    info!("beep up");
                } else if last_beep_hight - filtered_h > BEEP_INTERVAL {
                    last_beep_hight = filtered_h;
                    beep(&mut pwm, false).await;
                    info!("beep down");
                }
            }
            Err(_) => {
                warn!("Could not measure from BME280");
                d.delay_ms(100);
                if let Err(_) = ps.init_with_config(&mut d, bme_config) {
                    warn!("could not reinint BME280");
                    continue;
                };
                let _ = ps.common.set_normal_mode(&mut d);
                d.delay_ms(1000);
            }
        }

        Timer::after(LOOP_DT).await;
    }
}

#[embassy_executor::task]
async fn blink_led(mut led: Output<'static>) {
    loop {
        led.set_high();
        Timer::after_millis(100).await;
        led.set_low();
        Timer::after_millis(1000).await;
    }
}

/// if beep_up is true, beep up. If false, beep down
async fn beep<'d, T>(beeper_pin: &mut pwm::Pwm<'d, T>, beep_up: bool)
where
    T: pwm::Slice,
{
    let mut pwm_config = pwm::Config::default();
    let pwm_max_top = 0xffffu16;
    let pwm_min_top = pwm_max_top >> 4;
    // beeps with increasing freqency
    // final frequency is twice the starting freqency
    for i in 0..N_FREQUENCY_INCREASES {
        let factor = 1. + (3 * i) as f32 / N_FREQUENCY_INCREASES as f32;
        if beep_up {
            pwm_config.top = (pwm_max_top as f32 / factor) as u16;
        } else {
            pwm_config.top = (pwm_min_top as f32 * factor) as u16;
        }
        // 12.5% duty cycle
        pwm_config.compare_b = pwm_config.top >> 1;
        beeper_pin.set_config(&pwm_config);
        Timer::after(BEEP_TIME / N_FREQUENCY_INCREASES.into()).await
    }
    pwm_config.compare_b = 0;
    beeper_pin.set_config(&pwm_config);
}
