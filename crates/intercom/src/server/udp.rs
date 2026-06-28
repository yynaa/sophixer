use std::{
  collections::{HashMap, VecDeque},
  io::ErrorKind::WouldBlock,
  net::SocketAddr,
};

use serde_value::Value;
use tokio::net::UdpSocket;

use crate::{InterError, PrefixedMessage, server::InterServer};

pub struct UdpServer {
  sock: UdpSocket,

  messages: HashMap<String, VecDeque<(SocketAddr, Value)>>,
}

#[async_trait::async_trait]
impl InterServer for UdpServer {
  async fn start(addr: &str) -> Result<Self, InterError> {
    let sock = UdpSocket::bind(addr).await?;

    trace!("udp server started");

    Ok(Self {
      sock,
      messages: HashMap::new(),
    })
  }

  async fn stop(self) -> Result<(), InterError> {
    //nothing to do, just drop
    Ok(())
  }

  /// this udp implementation expects JSON messages of this format:
  /// ```json
  /// {
  ///   "message_type": "x",
  ///   "message": {...}
  /// }
  /// ```
  async fn fetch(&mut self) -> Result<(), InterError> {
    trace!("fetching messages");
    self.messages.clear();
    let mut buf = [0; 1024];
    loop {
      match self.sock.try_recv_from(&mut buf) {
        Ok((len, addr)) => {
          let msg = String::from_utf8_lossy(&buf[..len]).to_string();
          if let Ok(wrapped) = serde_json::from_str::<PrefixedMessage>(&msg) {
            if !self.messages.contains_key(&wrapped.prefix) {
              trace!("created unexistant prefix storage {}", wrapped.prefix);
              self
                .messages
                .insert(wrapped.prefix.clone(), VecDeque::new());
            }
            trace!("found {:?}", wrapped);
            self
              .messages
              .get_mut(&wrapped.prefix)
              .unwrap()
              .push_back((addr, wrapped.message));
          } else {
            warn!("couldn't read the following message: {}", msg);
          }
        }
        Err(e) => {
          if e.kind() == WouldBlock {
            trace!("no more messages");
            break;
          } else {
            warn!("unexpected error! {}", e);
            break;
          }
        }
      }
    }
    Ok(())
  }

  fn get(&self, prefix: String) -> Option<&VecDeque<(SocketAddr, Value)>> {
    trace!("got messages");
    self.messages.get(&prefix)
  }

  async fn send(&self, addr: SocketAddr, msg: String) -> Result<(), InterError> {
    trace!("sent to {}: {}", addr, msg);
    self.sock.send_to(&msg.into_bytes(), addr).await?;
    Ok(())
  }
}
