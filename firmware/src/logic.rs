use defmt::{error, info};
use embassy_stm32::gpio::Output;

use crate::protocol::Command;

pub async fn handle_command(command: Command, leds: &mut [Output<'_>]) {
    match command {
        Command::SetLED(index, value) => {
            // Handle setting a specific LED
            info!("Setting LED {} to {}", index, value);
            // Here you would add the logic to turn on the specific LED
            if (index as usize) < leds.len() {
                leds[index as usize].set_high();
            } else {
                error!("LED index {} out of range", index);
            }
        }
        Command::SetAll(value) => {
            if value {
                info!("Turning all LEDs ON");
                for led in leds.iter_mut() {
                    led.set_high();
                }
            } else {
                info!("Turning all LEDs OFF");
                for led in leds.iter_mut() {
                    led.set_low();
                }
            }
        }
        Command::Unknown => {
            error!("Received unknown command");
        }
    }
}
