#[macro_use]
extern crate log;
extern crate pretty_env_logger;
mod data;
mod model;
mod views;

use crate::data::read_set_data;
use crate::model::TinModel;
use crate::views::lpm3_matrix::ViewLPM3Matrix;
use anyhow::Result;
use argparse::{ArgumentParser, Store};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tin_drivers_midi::devices::launch_control_xl_mk2::driver::LCXL2Driver;
use tin_drivers_midi::devices::launchpad_mini_mk3::LPM3Driver;
use tin_drivers_midi::MidiDriver;
use tin_intercom::Server;

fn main() -> Result<()> {
    pretty_env_logger::init();

    let mut set_folder = ".".to_string();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("main server for Sophixer");
        ap.refer(&mut set_folder).add_argument(
            "set folder",
            Store,
            "folder where the set is located (see docs)",
        );
        ap.parse_args_or_exit();
    }
    trace!("loading set in: {set_folder:?}");

    let set = read_set_data(set_folder)?;
    let mut tin = TinModel::new(set);

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        info!("exiting...");
        r.store(false, Ordering::SeqCst);
    })?;

    let mut lpm3driver = LPM3Driver::connect()?;
    let mut lcxl2driver = LCXL2Driver::connect()?;
    let mut server = Server::start("127.0.0.1:3000")?;

    let mut view_lpm3_matrix = ViewLPM3Matrix::new();

    info!("running...");
    while running.load(Ordering::SeqCst) {
        server.fetch()?;

        view_lpm3_matrix.update(&mut tin, &mut lpm3driver)?;

        view_lpm3_matrix.draw(&tin, &mut lpm3driver)?;
    }

    lpm3driver.close()?;
    lcxl2driver.close()?;
    server.stop()?;

    Ok(())
}
