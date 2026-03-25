use std::{collections::HashMap, net::SocketAddr, rc::Rc};

use tin_core::song_data::{Set, Song};

pub struct RenoiseInstance {
  pub loaded_song: Option<Rc<Song>>,
}

impl RenoiseInstance {
  pub fn new() -> Self {
    Self { loaded_song: None }
  }
}

pub struct TinModel {
  pub set: Set,
  pub instances: HashMap<SocketAddr, RenoiseInstance>,
  pub instance_focus: Option<SocketAddr>,
}

impl TinModel {
  pub fn new(set: Set) -> Self {
    Self {
      set,
      instances: HashMap::new(),
      instance_focus: None,
    }
  }
}
