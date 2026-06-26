pub mod buttons;
pub mod channels;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::data::buttons::SongButton;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SongPattern {
  pub start: u64,
  pub loop_start: u64,
  pub loop_end: u64,
  pub color: [u8; 3],
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Song {
  pub name: String,
  pub authors: String,
  pub order: i64,
  pub color: [u8; 3],
  pub bpm: f64,
  pub patterns: HashMap<i64, SongPattern>,
  /// tuple represents x,y
  pub buttons: HashMap<(i64, i64), SongButton>,
}

impl Song {
  pub fn new(name: String, authors: String) -> Result<Song> {
    Ok(Song {
      name,
      authors,
      order: 0,
      color: [255, 255, 255],
      bpm: 125.,
      patterns: HashMap::new(),
      buttons: HashMap::new(),
    })
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Set {
  pub name: String,
  pub authors: String,
  pub stop_seq_pos: u64,
  pub songs: HashMap<String, Song>,
}

impl Set {
  pub fn new(name: String, authors: String) -> Result<Set> {
    Ok(Set {
      name,
      authors,
      stop_seq_pos: 0,
      songs: HashMap::new(),
    })
  }

  pub fn get_song_option(&self, song_id: Option<String>) -> Result<Option<&Song>> {
    if let Some(song_id) = song_id {
      let s = self
        .songs
        .get(&song_id)
        .ok_or(anyhow::Error::msg("couldn't find song id in songs"))?;
      Ok(Some(s))
    } else {
      Ok(None)
    }
  }
}
