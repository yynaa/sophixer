use std::{
  collections::VecDeque,
  io::ErrorKind::WouldBlock,
  net::{SocketAddr, ToSocketAddrs},
};

use tokio::net::UdpSocket;

use crate::{InterError, client::InterClient};

pub struct UdpClient {
  sock: UdpSocket,
  server_addr: SocketAddr,

  messages: VecDeque<Vec<u8>>,
}

#[async_trait::async_trait]
impl InterClient for UdpClient {
  async fn start(addr: &str) -> Result<Self, InterError> {
    let sock = UdpSocket::bind("0.0.0.0:0").await?;
    let server_addr = addr
      .to_socket_addrs()
      .map_err(InterError::IOError)?
      .next()
      .ok_or(InterError::NoSocketAddr(addr.to_string()))?;
    trace!("udp client started");

    Ok(Self {
      sock,
      server_addr,
      messages: VecDeque::new(),
    })
  }

  async fn stop(self) -> Result<(), InterError> {
    // nothing to do, just drop
    Ok(())
  }

  async fn fetch(&mut self) -> Result<(), InterError> {
    trace!("fetching messages");
    self.messages.clear();
    let mut buf = [0; 1024];
    loop {
      match self.sock.try_recv_from(&mut buf) {
        Ok((len, _addr)) => {
          let msg = buf[..len].to_vec();
          self.messages.push_back(msg);
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

  fn get(&self) -> Option<&VecDeque<Vec<u8>>> {
    if self.messages.len() > 0 {
      trace!("got {} message(s)", self.messages.len());
      Some(&self.messages)
    } else {
      trace!("got no messages");
      None
    }
  }

  async fn send(&self, msg: &[u8]) -> Result<(), InterError> {
    trace!("sent to {} len {}", self.server_addr, msg.len());
    self.sock.send_to(&msg, self.server_addr).await?;
    Ok(())
  }
}
