use anyhow::Result;
use bimap::BiMap;
use sophixer_core::song_data::{Set, Song, SongButtonAction};
use std::{collections::HashMap, net::SocketAddr};

pub struct RenoiseInstance {
  pub loaded_song: Option<String>,
  pub toggle_button_states: HashMap<(i64, i64), bool>,
}

impl RenoiseInstance {
  pub fn new() -> Self {
    Self {
      loaded_song: None,
      toggle_button_states: HashMap::new(),
    }
  }

  pub fn load_song(&mut self, song_id: &String, song: &Song) -> Result<()> {
    self.loaded_song = Some(song_id.clone());

    self.toggle_button_states.clear();
    for (y, section) in &song.sections {
      for (x, button) in &section.buttons {
        match button.action {
          SongButtonAction::ToggleChannels {
            channels: _,
            instant: _,
            color_off: _,
            color_on: _,
          } => {
            self.toggle_button_states.insert((*y, *x), true);
          }
          SongButtonAction::ToggleTrackPatterns {
            track_patterns: _,
            instant: _,
            color_off: _,
            color_on: _,
          } => {
            self.toggle_button_states.insert((*y, *x), true);
          }
        }
      }
    }

    Ok(())
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
