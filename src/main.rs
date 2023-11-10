use esp_idf_svc::hal::{prelude::Peripherals, gpio::PinDriver, delay::FreeRtos};
use log::info;
use anyhow::Result;

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("Hello, world!");

    let peripherals = Peripherals::take()?;
    let mut led = PinDriver::output(peripherals.pins.gpio12)?;

    loop {
        FreeRtos::delay_ms(500);

        led.toggle()?;
    }
}
