#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use anyhow::Result;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tin_drivers_midi::devices::launch_control_xl_mk2::driver::LCXL2Driver;
use tin_drivers_midi::devices::launchpad_mini_mk3::LPM3Driver;
use tin_drivers_midi::MidiDriver;
use tin_intercom::Server;

fn main() -> Result<()> {
    pretty_env_logger::init();

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        info!("exiting...");
        r.store(false, Ordering::SeqCst);
    })?;

    let mut lpm3driver = LPM3Driver::connect()?;
    let mut lcxl2driver = LCXL2Driver::connect()?;
    let mut server = Server::start("127.0.0.1:3000")?;

    info!("running...");
    while running.load(Ordering::SeqCst) {}

    lpm3driver.close()?;
    lcxl2driver.close()?;
    server.stop()?;

    Ok(())
}
