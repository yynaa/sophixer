use std::{
  collections::{HashMap, VecDeque},
  io::ErrorKind::WouldBlock,
  net::SocketAddr,
};

use tokio::net::UdpSocket;

use crate::{InterError, extract_prefix, server::InterServer};

pub struct UdpServer {
  sock: UdpSocket,

  messages: HashMap<u8, VecDeque<(SocketAddr, Vec<u8>)>>,
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
          let msg = buf[..len].to_vec();
          if let Ok((prefix, msg)) = extract_prefix(msg.clone()) {
            if !self.messages.contains_key(&prefix) {
              trace!("created unexistant prefix storage {}", prefix);
              self.messages.insert(prefix.clone(), VecDeque::new());
            }
            trace!("found message of length {}", msg.len());
            self
              .messages
              .get_mut(&prefix)
              .unwrap()
              .push_back((addr, msg));
          } else {
            warn!("couldn't read the following message: {:?}", msg);
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

  fn get(&self, prefix: u8) -> Option<&VecDeque<(SocketAddr, Vec<u8>)>> {
    trace!("got messages");
    self.messages.get(&prefix)
  }

  async fn send(&self, addr: SocketAddr, msg: &[u8]) -> Result<(), InterError> {
    trace!("sent to {} length {}", addr, msg.len());
    self.sock.send_to(msg, addr).await?;
    Ok(())
  }
}
