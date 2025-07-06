use cortex_m::asm::wfe;
use embassy_stm32::usb::{Driver, Instance};
use embassy_usb::{class::cdc_acm::CdcAcmClass, driver::EndpointError};

use defmt::{error, info};

use crate::protocol::Command;

pub async fn usb_task<'d, T: Instance + 'd>(class: &mut CdcAcmClass<'d, Driver<'d, T>>) -> ! {
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
        handle_command(Command::from_bytes(&buf), led).await;
    }
}
