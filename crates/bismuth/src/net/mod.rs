use anyhow::Result;
use intercom::client::InterClientCommunicator;
use sophixer_core::{
  messages::bismuth::{MessageFromBismuth, MessageToBismuth},
  song_data::{Set, Song},
};

use crate::{model::BismuthModel, net::client::Udp3dsClient};

pub mod client;

pub struct Communicator;
impl InterClientCommunicator<Udp3dsClient, MessageToBismuth, MessageFromBismuth> for Communicator {}

impl Communicator {
  pub fn update_model(model: &mut BismuthModel, client: &Udp3dsClient) -> Result<()> {
    let messages = Self::get_messages(client);
    if let Some(messages) = messages {
      for msg in messages {
        match msg {
          MessageToBismuth::Welcome => {
            info!("connected to tin");
          }
          MessageToBismuth::InitSet { name, authors } => {
            model.set = Some(Set::new(name.clone(), authors.clone())?);
            info!("received set {} by {}", name, authors);
          }
          MessageToBismuth::InitSong {
            id,
            name,
            authors,
            order,
            color,
          } => {
            let mut song = Song::new(name, authors, String::new())?;
            song.order = order;
            song.color = color;
            if let Some(set) = &mut model.set {
              set.songs.insert(id.clone(), song);
              info!("received song id {}", id)
            } else {
              warn!("ignored song init, since no set was initialized")
            }
          }
          MessageToBismuth::RenoiseInstanceAdded(id) => {
            model.renoise_instances.insert(id.clone());
            info!("renoise instance {} added", id);
          }
          MessageToBismuth::RenoiseInstanceRemoved(id) => {
            model.renoise_instances.remove(&id);
            info!("renoise instance {} removed", id);
          }
        }
      }
    }

    Ok(())
  }
}
