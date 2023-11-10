use std::cmp::min;

use esp_idf_svc::hal::{prelude::Peripherals, gpio::PinDriver, delay::FreeRtos, adc::{self, AdcDriver, AdcChannelDriver, attenuation}};
use log::info;
use anyhow::Result;

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("Hello, world!");

    let peripherals = Peripherals::take()?;
    
    let mut led = PinDriver::output(peripherals.pins.gpio12)?;
    
    let mut adc = AdcDriver::new(peripherals.adc2, &adc::config::Config::new().calibration(true))?;
    let mut adc_pin: AdcChannelDriver<{ attenuation::DB_11 }, _> = AdcChannelDriver::new(peripherals.pins.gpio14)?;

    loop {
        const MIN_BLINK_DELAY: u32 = 100;
        const MAX_BLINK_DELAY: u32 = 500;
        const BLINK_RANGE: u32 = MAX_BLINK_DELAY - MIN_BLINK_DELAY;

        const MIN_SENSOR: u16 = 500;
        const MAX_SENSOR: u16 = 3000;
        const SENSOR_RANGE: u16 = MAX_SENSOR - MIN_SENSOR;

        let value = adc.read(&mut adc_pin)?;
        info!("value: {value}");

        let value = min(value.saturating_sub(MIN_SENSOR), SENSOR_RANGE);
        
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]
        let delay = MIN_BLINK_DELAY + (f32::from(value) / f32::from(SENSOR_RANGE) * BLINK_RANGE as f32) as u32;

        info!("delay: {delay}");

        FreeRtos::delay_ms(delay);

        led.toggle()?;
    }
}