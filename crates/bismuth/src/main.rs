#[macro_use]
extern crate log;

use ansi_term::{Color, Style};
use anyhow::Result;
use ctru::prelude::*;
use env_logger::Builder;
use intercom::client::{InterClient, InterClientCommunicator};
use log::Level;
use sophixer_core::messages::bismuth::MessageFromBismuth;
use std::io::Write;
use std::{thread::sleep, time::Duration};

use crate::model::BismuthModel;
use crate::net::{client::Udp3dsClient, Communicator};
use crate::view::{render, update};

pub mod model;
pub mod net;
pub mod view;

fn main() {
  let mut builder = Builder::from_default_env();

  builder
    .format(|buf, record| {
      let color = match record.level() {
        Level::Warn => Color::Yellow,
        Level::Error => Color::Red,
        Level::Debug => Color::RGB(127, 127, 127),
        _ => Color::White,
      };

      writeln!(
        buf,
        "{}",
        Style::new()
          .fg(color)
          .paint(format!("[{}] {}", record.level(), record.args()))
      )
    })
    .filter(Some("bismuth"), log::LevelFilter::Trace)
    .filter(Some("intercom"), log::LevelFilter::Trace)
    .init();

  if let Err(e) = run() {
    let gfx = Gfx::new().expect("Couldn't obtain GFX controller");
    let bottom_console = Console::new(gfx.bottom_screen.borrow_mut());
    bottom_console.clear();
    bottom_console.select();
    println!("an error occurred!");
    println!("{}", e);
    sleep(Duration::from_secs(5));
  }
}

fn run() -> Result<()> {
  let gfx = Gfx::new()?;
  let mut hid = Hid::new()?;
  let _soc = Soc::new()?;
  let apt = Apt::new()?;
  let top_console = Console::new(gfx.top_screen.borrow_mut());
  let bottom_console = Console::new(gfx.bottom_screen.borrow_mut());

  let mut bismuth = BismuthModel::new();

  bottom_console.clear();
  bottom_console.select();

  let version = env!("CARGO_PKG_VERSION");
  info!("Bismuth {} started", version);

  info!("starting client... connecting...");
  let mut client = Udp3dsClient::start("10.44.209.146:3000")?;
  info!("client started, waving...");
  Communicator::send_message(&client, MessageFromBismuth::Hello)?;

  while apt.main_loop() {
    let mut rerender = false;

    bottom_console.select();

    if let Err(e) = client.fetch() {
      error!("ERROR: couldn't fetch client messages: {}", e.to_string());
    }
    rerender = rerender || Communicator::update_model(&mut bismuth, &client)?;

    hid.scan_input();
    let keys = hid.keys_down();

    if keys.contains(KeyPad::START) {
      break;
    }

    rerender = rerender || update(&keys, &mut bismuth, &client)?;

    if rerender {
      top_console.select();
      top_console.clear();
      render(&bismuth)?;
    }

    gfx.wait_for_vblank();
  }

  bottom_console.select();
  info!("exiting...");
  info!("sending goodbye message to tin...");
  Communicator::send_message(
    &client,
    sophixer_core::messages::bismuth::MessageFromBismuth::Goodbye,
  )?;
  info!("goodbye!");
  sleep(Duration::from_secs(1));
  Ok(())
}
