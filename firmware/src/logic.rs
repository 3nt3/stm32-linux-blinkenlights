use blinkenlights_protocol::Command;
use defmt::{error, info};
use embassy_stm32::gpio::Output;

pub async fn handle_command(command: Command, leds: &mut [Output<'_>]) {
    match command {
        Command::SetLED(index, value) => {
            // Handle setting a specific LED
            info!("Setting LED {} to {}", index, value);
            // Here you would add the logic to turn on the specific LED
            if (index as usize) < leds.len() {
                if value.into() {
                    leds[index as usize].set_high();
                } else {
                    leds[index as usize].set_low();
                }
            } else {
                error!("LED index {} out of range", index);
            }
        }
        Command::SetAll(value) => {
            info!(
                "Setting all LEDs to {}",
                if value.into() { "off" } else { "on" }
            );
            for led in leds.iter_mut() {
                if value.into() {
                    led.set_high();
                } else {
                    led.set_low();
                }
            }
        }
        Command::Unknown => {
            error!("Received unknown command");
        }
    }
}
