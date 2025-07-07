mod usb;

use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
};

use blinkenlights_protocol::{Command, Level};
use log::{error, info};

use crate::usb::{get_handle, handle_command};

fn main() {
    // set to RUST_LOG=info to see debug output
    dotenvy::dotenv().ok();

    env_logger::init();

    let mut handle = get_handle().expect("Failed to get USB handle");

    loop {
        // Read CPU usage percentage
        match get_cpu_sage_percent() {
            Some(usage) => {
                let n_leds = 9;
                let active_leds = ((usage / 100.0) * n_leds as f64).round() as usize;

                info!("CPU usage: {:.2}%, activating {} LEDs", usage, active_leds);

                for i in 0..n_leds {
                    if let Err(why) = handle_command(
                        &mut handle,
                        Command::SetLED(
                            i as u8,
                            if i < active_leds {
                                Level::High
                            } else {
                                Level::Low
                            },
                        ),
                    ) {
                        error!("Failed to set LED {} to high: {}", i, why);
                    }
                }
            }
            None => {
                eprintln!("Failed to read CPU usage.");
                handle.release_interface(1);
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(250));
    }
}

fn read_cpu_times() -> Option<(u64, u64)> {
    let file = File::open("/proc/stat").ok()?;
    let reader = BufReader::new(file);
    let line = reader.lines().next()?.ok()?; // First line is "cpu ..."

    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 5 {
        return None;
    }

    // Parse fields as u64
    let user = parts[1].parse::<u64>().ok()?;
    let nice = parts[2].parse::<u64>().ok()?;
    let system = parts[3].parse::<u64>().ok()?;
    let idle = parts[4].parse::<u64>().ok()?;
    let iowait = parts
        .get(5)
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);
    let irq = parts
        .get(6)
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);
    let softirq = parts
        .get(7)
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);
    let steal = parts
        .get(8)
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);

    let idle_all = idle + iowait;
    let total = user + nice + system + idle + iowait + irq + softirq + steal;

    Some((total, idle_all))
}

fn get_cpu_sage_percent() -> Option<f64> {
    let (total1, idle1) = read_cpu_times()?;
    std::thread::sleep(std::time::Duration::from_millis(100));
    let (total2, idle2) = read_cpu_times()?;

    let total_diff = total2 - total1;
    let idle_diff = idle2 - idle1;

    if total_diff == 0 {
        return Some(0.0);
    }

    let usage = 100.0 * (total_diff - idle_diff) as f64 / total_diff as f64;
    Some(usage)
}
