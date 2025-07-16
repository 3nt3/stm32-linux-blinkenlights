use blinkenlights_protocol::Command;
use defmt::{error, info};
use embassy_stm32::{
    gpio::OutputOpenDrain,
    usb::{Driver, Instance},
};
use embassy_usb::{class::cdc_acm::CdcAcmClass, driver::EndpointError};

use crate::logic;

pub async fn usb_task<'d, T: Instance + 'd>(
    class: &mut CdcAcmClass<'d, Driver<'d, T>>,
    leds: &mut [OutputOpenDrain<'_>],
) -> ! {
    loop {
        class.wait_connection().await;
        info!("USB connection established");
        handle_connection(class, leds).await;
        info!("USB connection lost");
    }
}

async fn handle_connection<'d, T: Instance + 'd>(
    class: &mut CdcAcmClass<'d, Driver<'d, T>>,
    leds: &mut [OutputOpenDrain<'_>],
) {
    let mut buf: [u8; 64] = [0; 64];
    loop {
        let n_res = class.read_packet(&mut buf).await;
        if let Err(why) = n_res {
            match why {
                EndpointError::BufferOverflow => {
                    // Handle buffer overflow, e.g., log or reset
                    error!("Buffer overflow in USB read");
                    continue;
                }
                EndpointError::Disabled => {
                    // Handle disabled endpoint, e.g., log or reset
                    error!("USB endpoint disabled");
                    continue;
                }
            }
        }

        let n = n_res.unwrap();

        if n == 0 {
            // No data read, continue to the next iteration
            info!("No data read from USB");
            continue;
        }

        info!("Received {} bytes from USB: {:?}", n, &buf[..n]);

        // Process the received data
        logic::handle_command(Command::from_bytes(&buf), leds).await;
    }
}
