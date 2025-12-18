mod common;
mod event_based;
mod poll_based;

use poll_based::daemon;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    daemon()
}
