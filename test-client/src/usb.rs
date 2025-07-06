use blinkenlights_protocol::Command;
use log::info;
use rusb::{Context, UsbContext};
use std::time::Duration;

pub fn handle_command(cmd: Command) -> anyhow::Result<()> {
    let context = Context::new()?;

    // Replace with your VID and PID
    let vid = 0xffff;
    let pid = 0xffff;

    // Find the device
    let devices = context.devices()?;
    let device = devices
        .iter()
        .find(|d| {
            let desc = d.device_descriptor().unwrap();
            desc.vendor_id() == vid && desc.product_id() == pid
        })
        .expect("Device not found");

    let handle = device.open()?;

    if handle.kernel_driver_active(0)? {
        // Detach kernel driver if it's active
        handle.detach_kernel_driver(0)?;
    }

    // Claim interface (check with lsusb -v)
    handle.claim_interface(0)?;

    // Prepare data to send
    let bytes = cmd.to_bytes();

    let endpoint_out = 0x02;

    let bytes_written = handle.write_bulk(endpoint_out, &bytes, Duration::from_secs(1))?;
    info!("Sent {} bytes", bytes_written);

    handle.release_interface(0)?;
    Ok(())
}
