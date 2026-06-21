#[macro_use]
extern crate log;
extern crate pretty_env_logger;
mod model;
mod servers;
mod views;

use crate::model::{LPM3View, TinModel};
use crate::servers::renoise::RenoiseCommunicator;
use crate::views::lpm3_matrix::ViewLPM3Matrix;
use crate::views::lpm3_songlist::ViewLPM3SongList;
use anyhow::Result;
use argparse::{ArgumentParser, Store};
use intercom::server::InterServer;
use intercom::server::udp::UdpServer;
use std::fs::read_to_string;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;
use tin_drivers_midi::MidiDriver;
use tin_drivers_midi::devices::launchpad_mini_mk3::LPM3Driver;

fn main() -> Result<()> {
  pretty_env_logger::init();

  let mut set_file = ".".to_string();
  {
    let mut ap = ArgumentParser::new();
    ap.set_description("main server for Sophixer");
    ap.refer(&mut set_file)
      .add_argument("set file", Store, "file of the set in ron notation");
    ap.parse_args_or_exit();
  }
  trace!("loading set in: {set_file:?}");

  let set_string = read_to_string(set_file)?;
  let set = ron::from_str(&set_string)?;

  let mut tin = TinModel::new(set);

  let running = Arc::new(AtomicBool::new(true));
  let r = running.clone();

  ctrlc::set_handler(move || {
    info!("exiting...");
    r.store(false, Ordering::SeqCst);
  })?;

  let mut lpm3driver = LPM3Driver::connect()?;

  let mut server = UdpServer::start("0.0.0.0:3000")?;

  let mut view_lpm3_songlist = ViewLPM3SongList::new(&tin);
  let mut view_lpm3_matrix = ViewLPM3Matrix::new();

  let mut instant = Instant::now();

  info!("running...");
  while running.load(Ordering::SeqCst) {
    let current_time = Instant::now();
    let delta_time = current_time - instant;

    server.fetch()?;
    RenoiseCommunicator::update_model(&mut tin, &server)?;

    let lpm3_inputs = lpm3driver.read()?;

    match &tin.lpm3view {
      LPM3View::Matrix => {
        view_lpm3_matrix.update(
          &delta_time,
          &mut tin,
          &mut lpm3driver,
          lpm3_inputs.clone(),
          &server,
        )?;
      }
      LPM3View::SongList => {
        view_lpm3_songlist.update(
          &delta_time,
          &mut tin,
          &mut lpm3driver,
          lpm3_inputs.clone(),
          &server,
        )?;
      }
    }

    lpm3driver.clear()?;

    match &tin.lpm3view {
      LPM3View::Matrix => {
        view_lpm3_matrix.draw(&tin, &mut lpm3driver)?;
      }
      LPM3View::SongList => {
        view_lpm3_songlist.draw(&tin, &mut lpm3driver)?;
      }
    }

    lpm3driver.push()?;

    instant = current_time;
  }

  lpm3driver.close()?;

  Ok(())
}
