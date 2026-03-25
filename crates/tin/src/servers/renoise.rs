use std::time::{Instant, SystemTime, UNIX_EPOCH};

use anyhow::Result;
use intercom::server::{udp::UdpServer, InterServerCommunicator};
use sophixer_core::messages::{
  bismuth::MessageToBismuth,
  renoise::{MessageFromRenoise, MessageToRenoise},
};

use crate::{
  model::{RenoiseInstance, TinModel},
  servers::bismuth::BismuthCommunicator,
};

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
            model.renoise_instances.insert(from, RenoiseInstance::new());
            let new_id = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
            model.renoise_instance_ids.insert(new_id.clone(), from);
            RenoiseCommunicator::send_message(server, from, MessageToRenoise::Welcome)?;
            if let Some(bismuth) = model.bismuth_instance {
              BismuthCommunicator::send_message(
                server,
                bismuth,
                MessageToBismuth::RenoiseInstanceAdded(new_id),
              )?;
            }
            info!("renoise instance from {} connected", from);
          }
          MessageFromRenoise::Goodbye => {
            model.renoise_instances.remove(&from);
            if let Some(bismuth) = model.bismuth_instance {
              if let Some(id) = model.renoise_instance_ids.get_by_right(&from) {
                BismuthCommunicator::send_message(
                  server,
                  bismuth,
                  MessageToBismuth::RenoiseInstanceRemoved(id.clone()),
                )?;
              }
            }
            model.renoise_instance_ids.remove_by_right(&from);
            info!("renoise instance from {} disconnected", from);
          }
        }
      }
    }

    Ok(())
  }
}
