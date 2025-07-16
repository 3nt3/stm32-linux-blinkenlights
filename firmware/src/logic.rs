use blinkenlights_protocol::Command;
use defmt::{error, info};
use embassy_stm32::gpio::OutputOpenDrain;

pub async fn handle_command(command: Command, leds: &mut [OutputOpenDrain<'_>]) {
    match command {
        Command::SetLED(index, value) => {
            // Handle setting a specific LED
            info!("Setting LED {} to {}", index, value);
            // Here you would add the logic to turn on the specific LED
            if (index as usize) < leds.len() {
                // we reverse the logic levels because we switch the cathode
                if value.into() {
                    leds[index as usize].set_low();
                } else {
                    leds[index as usize].set_high();
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
                    led.set_low();
                } else {
                    led.set_high();
                }
            }
        }
        Command::Unknown => {
            error!("Received unknown command");
        }
    }
}
