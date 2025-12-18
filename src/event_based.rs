use crate::common::read_bat_status;
use nix::poll::{poll, PollFd, PollFlags, PollTimeout};
use std::os::fd::AsFd;
use udev::MonitorBuilder;

const POWER_SUPPLY_SUBSYSTEM: &str = "power_supply";
const EVENT_CHANGE_STR: &str = "change";

/// Event driven daemon
/// Blocks on the udev monitor on power system events.
/// When changes occur, poll the battery files and generate notification as needed
pub fn daemon() -> Result<(), Box<dyn std::error::Error>> {
    let monitor_sock = MonitorBuilder::new()?
        .match_subsystem(POWER_SUPPLY_SUBSYSTEM)?
        .listen()?;

    let fd = monitor_sock.as_fd();
    let mut fds = [PollFd::new(fd, PollFlags::POLLIN)];

    loop {
        let res = poll(&mut fds, PollTimeout::NONE)?;
        if res < 0 {
            panic!("Polling sock failed");
        }
        let event = match monitor_sock.iter().next() {
            Some(evt) => evt,
            None => {
                panic!("Polled from sock but no event");
            }
        };
        println!("{:#?}", event);
    }
}
