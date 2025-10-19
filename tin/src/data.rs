use anyhow::Result;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::read_to_string;

#[derive(Deserialize, Debug)]
pub struct SongTrack {
    tracks: HashSet<u8>,
}

#[derive(Deserialize, Debug)]
pub struct SongKnob {
    track: u8,
    effect: u8,
    parameter: u8,
    color: (u8, u8),
}

#[derive(Deserialize, Debug)]
pub struct SongMatrix {
    color: (u8, u8, u8),
    action: SongMatrixAction,
}

#[derive(Deserialize, Debug)]
pub enum SongMatrixAction {
    Pattern(SongMatrixActionPattern),
    Sound(SongMatrixActionSound),
}

#[derive(Deserialize, Debug)]
pub struct SongMatrixActionPattern {
    tracks: HashSet<u8>,
    pattern: u8,
    length: Option<u8>,
}

#[derive(Deserialize, Debug)]
pub struct SongMatrixActionSound {
    channel: u8,
    note: u8,
    reactive: bool,
}

#[derive(Deserialize, Debug)]
pub struct Song {
    name: String,
    authors: String,
    path: String,
    order: i64,
    #[serde(default)]
    tracks: HashMap<i64, SongTrack>,
    #[serde(default)]
    knobs: HashMap<i64, HashMap<i64, SongKnob>>,
    #[serde(default)]
    matrix: HashMap<i64, HashMap<i64, SongMatrix>>,
}

#[derive(Deserialize, Debug)]
pub struct Set {
    name: String,
    authors: String,
    #[serde(skip)]
    songs: HashMap<String, Song>,
}

pub fn read_set_data(set_folder: String) -> Result<Set> {
    trace!("reading file {}", set_folder.clone() + "metadata.toml");
    let metadata_string = read_to_string(set_folder.clone() + "metadata.toml")?;
    let mut set: Set = toml::from_str(&metadata_string)?;

    let paths = fs::read_dir(&set_folder)?;

    for p in paths {
        let file_name = p?.file_name().to_string_lossy().to_string();
        if file_name.ends_with(".toml") && file_name != "metadata.toml" {
            trace!("reading file {}", set_folder.clone() + &file_name);
            let song_string = read_to_string(set_folder.clone() + &file_name)?;
            let song: Song = toml::from_str(&song_string)?;
            let cropped_name = file_name[..file_name.len() - 5].to_string();
            set.songs.insert(cropped_name, song);
        }
    }

    Ok(set)
}
