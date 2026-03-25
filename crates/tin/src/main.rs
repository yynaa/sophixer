#[macro_use]
extern crate log;
extern crate pretty_env_logger;
mod model;
mod servers;
mod views;

use crate::model::TinModel;
use crate::servers::bismuth::BismuthCommunicator;
use crate::servers::renoise::RenoiseCommunicator;
use crate::views::lpm3_matrix::ViewLPM3Matrix;
use anyhow::Result;
use argparse::{ArgumentParser, Store};
use intercom::server::udp::UdpServer;
use intercom::server::InterServer;
use sophixer_core::song_data::Set;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;
use tin_drivers_midi::devices::launch_control_xl_mk2::driver::LCXL2Driver;
use tin_drivers_midi::devices::launchpad_mini_mk3::LPM3Driver;
use tin_drivers_midi::MidiDriver;

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

  let set = Set::from_folder(set_folder)?;
  let mut tin = TinModel::new(set);

  let running = Arc::new(AtomicBool::new(true));
  let r = running.clone();

  ctrlc::set_handler(move || {
    info!("exiting...");
    r.store(false, Ordering::SeqCst);
  })?;

  let mut lpm3driver = LPM3Driver::connect()?;
  let mut lcxl2driver = LCXL2Driver::connect()?;

  let mut server = UdpServer::start("0.0.0.0:3000")?;

  let mut view_lpm3_matrix = ViewLPM3Matrix::new();

  let mut instant = Instant::now();

  info!("running...");
  while running.load(Ordering::SeqCst) {
    let current_time = Instant::now();
    let delta_time = current_time - instant;

    server.fetch()?;
    RenoiseCommunicator::update_model(&mut tin, &server)?;
    BismuthCommunicator::update_model(&mut tin, &server)?;

    view_lpm3_matrix.update(&delta_time, &mut tin, &mut lpm3driver)?;

    lpm3driver.clear()?;
    lcxl2driver.clear()?;

    view_lpm3_matrix.draw(&tin, &mut lpm3driver)?;

    lpm3driver.push()?;
    lcxl2driver.push()?;

    instant = current_time;
  }

  lpm3driver.close()?;
  lcxl2driver.close()?;

  Ok(())
}
