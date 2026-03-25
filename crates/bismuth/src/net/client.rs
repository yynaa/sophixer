use std::{
  collections::VecDeque,
  net::{Ipv4Addr, SocketAddrV4, UdpSocket},
};

use intercom::{client::InterClient, InterError};

pub struct Udp3dsClient {
  stream: UdpSocket,
  server_addr: String,
  messages: VecDeque<String>,
}

impl InterClient for Udp3dsClient {
  fn start(addr: &str) -> Result<Self, intercom::InterError> {
    let address = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 3000);
    let stream = UdpSocket::bind(&address).map_err(InterError::IOError)?;
    stream.set_nonblocking(true).map_err(InterError::IOError)?;

    Ok(Self {
      stream,
      server_addr: addr.to_string(),
      messages: VecDeque::new(),
    })
  }

  fn stop(self) -> Result<(), InterError> {
    Ok(())
  }

  fn send(&self, msg: String) -> Result<(), InterError> {
    self
      .stream
      .send_to(&msg.into_bytes(), self.server_addr.clone())
      .map_err(InterError::IOError)?;

    Ok(())
  }

  fn fetch(&mut self) -> Result<(), InterError> {
    self.messages.clear();
    let mut buf = [0u8; 2048];
    match self.stream.recv(&mut buf) {
      Ok(amt) => {
        let received = String::from_utf8_lossy(&buf[..amt]);
        let noend = received.trim_end_matches(';');
        self.messages.push_back(noend.to_string());
      }
      Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
      Err(e) => {
        error!("error: {:?}", e.to_string());
      }
    }
    Ok(())
  }

  fn get(&self) -> Option<&VecDeque<String>> {
    if self.messages.len() > 0 {
      Some(&self.messages)
    } else {
      None
    }
  }
}
