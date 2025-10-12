#[macro_use]
extern crate log;
extern crate pretty_env_logger;
use anyhow::Result;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tin_drivers_midi::devices::launchpad_mini_mk3::driver::LPM3Driver;
use tin_drivers_midi::devices::launchpad_mini_mk3::input::LPM3InputMessage;
use tin_drivers_midi::devices::launchpad_mini_mk3::visual::LPM3Visual;
use tin_drivers_midi::MidiDriver;

fn main() -> Result<()> {
    pretty_env_logger::init();

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        info!("exiting...");
        r.store(false, Ordering::SeqCst);
    })?;

    let mut lm3driver = LPM3Driver::connect()?;
    lm3driver.clear()?;

    info!("Tin running...");
    while running.load(Ordering::SeqCst) {
        let mut d = lm3driver.read()?;
        while let Some(msg) = d.pop_front() {
            match msg {
                LPM3InputMessage::KeyPressed(pos) => lm3driver.add(LPM3Visual::Static(pos, 3))?,
                LPM3InputMessage::KeyReleased(pos) => lm3driver.add(LPM3Visual::Off(pos))?,
            }
        }
        lm3driver.push()?;
    }

    lm3driver.close()?;
    Ok(())
}
