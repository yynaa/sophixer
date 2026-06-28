use anyhow::Result;
use intercom::server::{InterServerCommunicator, udp::UdpServer};
use sophixer_core::messages::renoise::{
  from::{MessageFromRenoise, message_from_renoise::FromMessage},
  to::{MessageToRenoise, Welcome, message_to_renoise::ToMessage},
};

use crate::model::TinModel;

// pub struct RenoiseCommunicator {}
// impl InterServerCommunicator<UdpServer, MessageFromRenoise, MessageToRenoise>
//   for RenoiseCommunicator
// {
// }

pub type RenoiseCommunicator =
  InterServerCommunicator<UdpServer, MessageFromRenoise, MessageToRenoise>;

pub async fn update_model(model: &mut TinModel, server: &UdpServer) -> Result<()> {
  let messages = RenoiseCommunicator::get_messages(server);
  if let Some(messages) = messages {
    for (from, msg) in messages {
      match msg.unpack()? {
        FromMessage::Hello(_) => {
          model.renoise_socket = Some(from);
          info!("renoise connected");
          RenoiseCommunicator::send_message(
            server,
            from,
            MessageToRenoise::build(ToMessage::Welcome(Welcome {}))?,
          )
          .await?;
        }
        FromMessage::Goodbye(_) => {
          model.renoise_socket = None;
          info!("renoise disconnected");
        }
      }
    }
  }

  Ok(())
}
