use crate::common::read_bat_status;
use std::thread;
use std::time::Duration;

const CHECK_INTVL_SEC: Duration = Duration::from_secs(300);

/// Daemon that runs to poll the battery status and percentage, notify when needed
pub fn daemon() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        read_bat_status()?;
        thread::sleep(CHECK_INTVL_SEC);
    }
}
