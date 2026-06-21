use anyhow::Result;
use intercom::server::{udp::UdpServer, InterServerCommunicator};
use sophixer_core::messages::renoise::{MessageFromRenoise, MessageToRenoise};

use crate::model::TinModel;

pub struct RenoiseCommunicator {}
impl InterServerCommunicator<UdpServer, MessageFromRenoise, MessageToRenoise>
  for RenoiseCommunicator
{
}

impl RenoiseCommunicator {
  pub fn update_model(model: &mut TinModel, server: &UdpServer) -> Result<()> {
    let messages = RenoiseCommunicator::get_messages(server);
    if let Some(messages) = messages {
      for (from, msg) in messages {
        match msg {
          MessageFromRenoise::Hello => {
            model.renoise_socket = Some(from);
            info!("renoise connected");
          }
          MessageFromRenoise::Goodbye => {
            model.renoise_socket = None;
            info!("renoise disconnected");
          }
        }
      }
    }

    Ok(())
  }
}
