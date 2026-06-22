use sophixer_core::data::{
  Set,
  buttons::{ActionDescriptor, SongButtonActionValue},
};
use std::{collections::HashMap, net::SocketAddr};

pub enum LPM3View {
  SongList,
  Matrix,
}

pub struct TinModel {
  pub set: Set,

  pub lpm3view: LPM3View,

  pub renoise_socket: Option<SocketAddr>,
  pub current_song: Option<String>,

  pub button_states: HashMap<(String, i64, i64), SongButtonActionValue>,
}

impl TinModel {
  pub fn new(set: Set) -> Self {
    let mut button_states = HashMap::new();

    for (song_id, song) in &set.songs {
      for ((bx, by), button) in &song.buttons {
        button_states.insert((song_id.clone(), *bx, *by), button.action.get_default());
      }
    }

    Self {
      set,
      lpm3view: LPM3View::SongList,
      renoise_socket: None,
      current_song: None,
      button_states,
    }
  }
}
