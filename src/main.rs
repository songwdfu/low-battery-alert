use notify_rust::{Notification, Timeout};
use std::{fs, thread, time::Duration};

const CAPACITY_PATH: &str = "/sys/class/power_supply/BAT0/capacity";
const STATUS_PATH: &str = "/sys/class/power_supply/BAT0/status";

const DISCHARGING: &str = "Discharging";
const LOW_PERCENTAGE: u8 = 30;
const CRITICAL_PERCENTAGE: u8 = 15;
const CHECK_INTVL_SEC: Duration = Duration::from_secs(300);
const NOTIFY_INTVL_SEC: Duration = Duration::from_secs(300);

const NOTIF_TIMEOUT_SEC: u32 = 15;

/// Get battery percentage from the capacity file
fn get_battery_percentage() -> std::io::Result<u8> {
    let contents = fs::read_to_string(CAPACITY_PATH)?;
    Ok(contents.trim().parse::<u8>().unwrap_or(100))
}

/// Get battery status from the status file
fn get_battery_status() -> std::io::Result<String> {
    let contents = fs::read_to_string(STATUS_PATH)?;
    Ok(contents.trim().to_string())
}

/// Daemon that runs to poll the battery status and percentage, notify when needed
fn daemon() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let percentage = get_battery_percentage()?;
        let status = get_battery_status()?;
        if status == DISCHARGING {
            if percentage < CRITICAL_PERCENTAGE {
                Notification::new()
                    .summary("Critical Battery Percentage")
                    .body(&format!("Battery percentage: {}%", percentage))
                    .timeout(Timeout::Milliseconds(NOTIF_TIMEOUT_SEC * 1000))
                    .show()?;
            } else if percentage < LOW_PERCENTAGE {
                Notification::new()
                    .summary("Low Battery Percentage")
                    .body(&format!("Battery percentage: {}%", percentage))
                    .timeout(Timeout::Milliseconds(NOTIF_TIMEOUT_SEC * 1000))
                    .show()?;
            } else {
                Notification::new()
                    .summary("Battery Discharging")
                    .body(&format!("Battery percentage: {}%", percentage))
                    .timeout(Timeout::Milliseconds(NOTIF_TIMEOUT_SEC * 1000))
                    .show()?;
            }
            thread::sleep(NOTIFY_INTVL_SEC);
        } else {
            thread::sleep(CHECK_INTVL_SEC);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    daemon()
}
