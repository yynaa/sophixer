use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::fs::{self, create_dir_all, File};
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
pub enum SongButtonAction {
  ToggleChannels {
    channels: HashSet<u64>,
    instant: bool,
    color_off: (u8, u8, u8),
    color_on: (u8, u8, u8),
  },
  ToggleTrackPatterns {
    track_patterns: HashSet<(u64, u64)>,
    instant: bool,
    color_off: (u8, u8, u8),
    color_on: (u8, u8, u8),
  },
}

impl SongButtonAction {
  pub fn default_toggle_channels() -> Result<SongButtonAction> {
    Ok(SongButtonAction::ToggleChannels {
      channels: HashSet::new(),
      instant: false,
      color_off: (127, 0, 0),
      color_on: (0, 0, 127),
    })
  }

  pub fn default_toggle_track_patterns() -> Result<SongButtonAction> {
    Ok(SongButtonAction::ToggleTrackPatterns {
      track_patterns: HashSet::new(),
      instant: false,
      color_off: (127, 0, 0),
      color_on: (0, 0, 127),
    })
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SongButton {
  pub action: SongButtonAction,
}

impl SongButton {
  pub fn new(action: SongButtonAction) -> Result<SongButton> {
    Ok(Self { action })
  }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SongSection {
  pub start: u64,
  pub length: u64,
  pub color: (u8, u8, u8),
  pub buttons: HashMap<i64, SongButton>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Song {
  pub name: String,
  pub authors: String,
  pub path: String,
  pub order: i64,
  pub color: (u8, u8, u8),
  pub sections: HashMap<i64, SongSection>,
}

impl Song {
  pub fn new(name: String, authors: String, path: String) -> Result<Song> {
    Ok(Song {
      name,
      authors,
      path,
      order: 0,
      color: (127, 127, 127),
      sections: HashMap::new(),
    })
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Set {
  pub name: String,
  pub authors: String,
  #[serde(skip)]
  pub songs: HashMap<String, Song>,
}

impl Set {
  pub fn new(name: String, authors: String) -> Result<Set> {
    Ok(Set {
      name,
      authors,
      songs: HashMap::new(),
    })
  }

  pub fn from_folder(set_folder: String) -> Result<Set> {
    trace!("reading file {}", set_folder.clone() + "/metadata.set");
    let metadata_string = read_to_string(set_folder.clone() + "/metadata.set")?;
    let mut set: Set = ron::from_str(&metadata_string)?;

    let paths = fs::read_dir(&set_folder)?;

    for p in paths {
      let file_name = p?.file_name().to_string_lossy().to_string();
      if file_name.ends_with(".song") {
        trace!("reading file {}", set_folder.clone() + &file_name);
        let song_string = read_to_string(set_folder.clone() + "/" + &file_name)?;
        let song: Song = ron::from_str(&song_string)?;
        let cropped_name = file_name[..file_name.len() - 5].to_string();
        set.songs.insert(cropped_name, song);
      }
    }

    Ok(set)
  }

  pub fn save_in_folder(&self, set_folder: String) -> Result<()> {
    create_dir_all(set_folder.clone())?;

    let set_string = ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default())?;
    File::create(set_folder.clone() + "/metadata.set")?.write_all(&set_string.into_bytes())?;

    for (song_name, song) in &self.songs {
      let song_string = ron::ser::to_string_pretty(&song, ron::ser::PrettyConfig::default())?;
      File::create(set_folder.clone() + "/" + &song_name.clone() + ".song")?
        .write_all(&song_string.into_bytes())?;
    }

    Ok(())
  }
}
