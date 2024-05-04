#![no_std]
#![no_main]
use bme280;
use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::{gpio, i2c};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::Delay;
use embassy_time::{Duration, Ticker, Timer};
use embedded_hal::delay::DelayNs;

use gpio::{Level, Output};
use {defmt_rtt as _, panic_probe as _};

type LedType = Mutex<ThreadModeRawMutex, Option<Output<'static>>>;
static LED: LedType = Mutex::new(None);

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // add led blinking
    let led = Output::new(p.PIN_25, Level::High);
    {
        *(LED.lock().await) = Some(led);
    }
    let dt = 100 * 1_000_000;
    let k = 1.003;

    unwrap!(spawner.spawn(toggle_led(&LED, Duration::from_nanos(dt))));
    unwrap!(spawner.spawn(toggle_led(
        &LED,
        Duration::from_nanos((dt as f64 * k) as u64)
    )));

    Timer::after(Duration::from_millis(10_000)).await;

    // init the bme280 i2c
    let (sda, sck) = (p.PIN_0, p.PIN_1);
    let pressure_i2c = i2c::I2c::new_blocking(p.I2C0, sck, sda, i2c::Config::default());

    let mut bme = bme280::i2c::BME280::new_primary(pressure_i2c);
    let mut delay = Delay {};
    // delay.delay_ms(10000);

    match bme.init(&mut delay) {
        Ok(_) => {
            info!("bme init success");
            let mut led = LED.lock().await;
            if let Some(pin) = led.as_mut() {
                for _ in 0..10 {
                    pin.toggle();
                    delay.delay_ms(100);
                }
            }
        }
        Err(_) => {
            warn!("bme init fail");
            let mut led = LED.lock().await;
            if let Some(pin) = led.as_mut() {
                for i in 0..20 {
                    pin.toggle();
                    delay.delay_ms(i * 100);
                }
            }
            crate::panic!()
        }
    }

    loop {
        let _p = bme.measure(&mut delay).unwrap().pressure;
    }
}

#[embassy_executor::task(pool_size = 2)]
async fn toggle_led(led: &'static LedType, delay: Duration) {
    let mut ticker = Ticker::every(delay);
    loop {
        {
            let mut led_unlocked = led.lock().await;
            if let Some(pin_ref) = led_unlocked.as_mut() {
                pin_ref.toggle();
            }
        }
        ticker.next().await;
    }
}
