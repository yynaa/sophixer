use anyhow::Result;
use intercom::server::{udp::UdpServer, InterServerCommunicator};
use sophixer_core::messages::bismuth::{MessageFromBismuth, MessageToBismuth};

use crate::model::TinModel;

pub struct BismuthCommunicator {}
impl InterServerCommunicator<UdpServer, MessageFromBismuth, MessageToBismuth>
  for BismuthCommunicator
{
}

impl BismuthCommunicator {
  pub fn update_model(model: &mut TinModel, server: &UdpServer) -> Result<()> {
    let messages = BismuthCommunicator::get_messages(server);
    if let Some(messages) = messages {
      for (from, msg) in messages {
        match msg {
          MessageFromBismuth::Hello => {
            model.bismuth_instance = Some(from);
            BismuthCommunicator::send_message(server, from, MessageToBismuth::Welcome)?;
            BismuthCommunicator::send_message(server, from, model.set.clone().into())?;
            for (id, song) in &model.set.songs {
              BismuthCommunicator::send_message(
                server,
                from,
                MessageToBismuth::InitSong {
                  id: id.clone(),
                  name: song.name.clone(),
                  authors: song.authors.clone(),
                  order: song.order,
                  color: song.color,
                },
              )?;
            }
            for id in model.renoise_instance_ids.left_values() {
              BismuthCommunicator::send_message(
                server,
                from,
                MessageToBismuth::RenoiseInstanceAdded(id.clone()),
              )?;
            }
            info!("bismuth instance from {} connected", from);
          }
          MessageFromBismuth::Goodbye => {
            model.bismuth_instance = None;
            info!("bismuth instance from {} disconnected", from);
          }
        }
      }
    }

    Ok(())
  }
}
