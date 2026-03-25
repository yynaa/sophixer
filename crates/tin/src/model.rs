use std::{collections::HashMap, net::SocketAddr, rc::Rc};

use bimap::BiMap;
use sophixer_core::song_data::{Set, Song};

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
  pub bismuth_instance: Option<SocketAddr>,
  pub renoise_instances: HashMap<SocketAddr, RenoiseInstance>,
  pub renoise_instance_ids: BiMap<u64, SocketAddr>,
  pub renoise_instance_focus: Option<SocketAddr>,
}

impl TinModel {
  pub fn new(set: Set) -> Self {
    Self {
      set,
      bismuth_instance: None,
      renoise_instances: HashMap::new(),
      renoise_instance_ids: BiMap::new(),
      renoise_instance_focus: None,
    }
  }
}
