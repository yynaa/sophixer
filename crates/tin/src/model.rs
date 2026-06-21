use sophixer_core::data::{Set, buttons::SongButtonActionDefault};
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

  pub toggle_button_states: HashMap<(String, i64, i64), bool>,
  pub cycle_button_states: HashMap<(String, i64, i64), usize>,
}

impl TinModel {
  pub fn new(set: Set) -> Self {
    let mut toggle_button_states = HashMap::new();
    let mut cycle_button_states = HashMap::new();

    for (song_id, song) in &set.songs {
      for ((bx, by), button) in &song.buttons {
        match button.action.get_default() {
          SongButtonActionDefault::None => {}
          SongButtonActionDefault::Boolean(b) => {
            toggle_button_states.insert((song_id.clone(), *bx, *by), b);
          }
          SongButtonActionDefault::Number(n) => {
            cycle_button_states.insert((song_id.clone(), *bx, *by), n);
          }
        }
      }
    }

    Self {
      set,
      lpm3view: LPM3View::SongList,
      renoise_socket: None,
      current_song: None,
      toggle_button_states,
      cycle_button_states,
    }
  }
}
