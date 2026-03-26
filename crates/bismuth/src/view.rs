use anyhow::Result;
use ctru::prelude::KeyPad;
use intercom::client::InterClientCommunicator;
use sophixer_core::{messages::bismuth::MessageFromBismuth, song_data::Song};

use crate::{
  model::BismuthModel,
  net::{client::Udp3dsClient, Communicator},
};

pub fn update(keys: &KeyPad, model: &mut BismuthModel, client: &Udp3dsClient) -> Result<bool> {
  let mut modified = false;

  if let Some(set) = &model.set {
    if keys.contains(KeyPad::DPAD_LEFT) || keys.contains(KeyPad::DPAD_RIGHT) {
      if set.songs.len() > 0 {
        if let Some(current_song_id) = &model.song_selector {
          let current_song = set.songs.get(current_song_id).ok_or(anyhow::Error::msg(
            "couldn't find song from current song selected",
          ))?;
          if keys.contains(KeyPad::DPAD_LEFT) {
            let mut sorted = set
              .songs
              .iter()
              .filter(|f| f.1.order < current_song.order)
              .collect::<Vec<(&String, &Song)>>();
            sorted.sort_by_key(|f| f.1.order);
            model.song_selector = sorted.last().map(|f| f.0.clone());
          } else {
            let mut sorted = set
              .songs
              .iter()
              .filter(|f| f.1.order > current_song.order)
              .collect::<Vec<(&String, &Song)>>();
            sorted.sort_by_key(|f| f.1.order);

            model.song_selector = sorted.first().map(|f| f.0.clone());
          }
        } else {
          let mut sorted = set.songs.iter().collect::<Vec<(&String, &Song)>>();
          sorted.sort_by_key(|f| f.1.order);
          if keys.contains(KeyPad::DPAD_LEFT) {
            model.song_selector = Some(sorted.last().unwrap().0.clone());
          } else {
            model.song_selector = Some(sorted.first().unwrap().0.clone());
          }
        }
      } else {
        model.song_selector = None;
      }
      modified = true;
    }
  }

  if keys.contains(KeyPad::DPAD_UP) || keys.contains(KeyPad::DPAD_DOWN) {
    if model.renoise_instances.len() > 0 {
      if let Some(current_ri) = &model.renoise_instance_selector {
        if keys.contains(KeyPad::DPAD_UP) {
          let mut sorted = model
            .renoise_instances
            .iter()
            .filter(|f| **f < *current_ri)
            .collect::<Vec<&u64>>();
          sorted.sort_by_key(|f| **f);
          model.renoise_instance_selector = sorted.last().map(|f| **f);
        } else {
          let mut sorted = model
            .renoise_instances
            .iter()
            .filter(|f| **f > *current_ri)
            .collect::<Vec<&u64>>();
          sorted.sort_by_key(|f| **f);
          model.renoise_instance_selector = sorted.first().map(|f| **f);
        }
      } else {
        let mut sorted = model.renoise_instances.iter().collect::<Vec<&u64>>();
        sorted.sort_by_key(|f| **f);
        if keys.contains(KeyPad::DPAD_UP) {
          model.renoise_instance_selector = Some(**sorted.last().unwrap());
        } else {
          model.renoise_instance_selector = Some(**sorted.first().unwrap());
        }
      }
    } else {
      model.renoise_instance_selector = None;
    }
    modified = true;
  }

  if keys.contains(KeyPad::A) {
    if let Some(current_song) = &model.song_selector {
      if let Some(current_ri) = &model.renoise_instance_selector {
        Communicator::send_message(
          client,
          MessageFromBismuth::LoadSong(*current_ri, current_song.clone()),
        )?;
      }
    }
  }

  Ok(modified)
}

pub fn render(model: &BismuthModel) -> Result<()> {
  if let Some(set) = &model.set {
    if let Some(current_song_id) = &model.song_selector {
      let current_song = set
        .songs
        .get(current_song_id)
        .ok_or(anyhow::Error::msg("couldn't get song from id"))?;
      println!("{}", current_song.name);
    } else {
      println!("-");
    }
  } else {
    println!("no set loaded, can't choose a song")
  }

  if let Some(current_ri) = &model.renoise_instance_selector {
    println!("{}", current_ri);
  } else {
    println!("-");
  }

  Ok(())
}
