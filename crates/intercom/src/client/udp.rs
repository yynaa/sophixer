use crate::client::InterClient;
use crate::InterError;
use log::{error, trace};
use std::collections::VecDeque;
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::thread;
use std::thread::JoinHandle;

type InternalSignal = String;

pub struct UdpClient {
  stop_flag: Arc<AtomicBool>,

  handle_reader: JoinHandle<()>,
  handle_sender: JoinHandle<()>,

  rx_reader: mpsc::Receiver<InternalSignal>,
  tx_sender: mpsc::Sender<InternalSignal>,

  messages: VecDeque<String>,
}

fn udp_reader(
  socket: UdpSocket,
  server_addr: SocketAddr,
  tx: mpsc::Sender<InternalSignal>,
  stop_flag: Arc<AtomicBool>,
) {
  let mut buf = [0; 1024];
  while !stop_flag.load(Ordering::Relaxed) {
    match socket.recv_from(&mut buf) {
      Ok((len, src)) => {
        if src == server_addr {
          let msg = String::from_utf8_lossy(&buf[..len]).to_string();
          match tx.send(msg.clone()) {
            Ok(()) => {
              trace!("received {} from server", msg);
            }
            Err(e) => {
              error!("mpsc send error: {e:?}");
            }
          }
        }
      }
      Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
      Err(e) => {
        error!("udp error: {e:?}");
      }
    }
  }
}

fn udp_sender(
  socket: UdpSocket,
  server_addr: SocketAddr,
  rx: mpsc::Receiver<InternalSignal>,
  stop_flag: Arc<AtomicBool>,
) {
  while !stop_flag.load(Ordering::Relaxed) {
    match rx.try_recv() {
      Ok(msg) => match socket.send_to(msg.as_bytes(), server_addr) {
        Ok(_) => {
          trace!("sent {} to server", msg);
        }
        Err(e) => {
          error!("couldn't send message on socket: {e:?}");
        }
      },
      Err(mpsc::TryRecvError::Empty) => {}
      Err(mpsc::TryRecvError::Disconnected) => {
        break;
      }
    }
  }
}

impl InterClient for UdpClient {
  fn start(addr: &str) -> Result<Self, InterError> {
    let socket = UdpSocket::bind("0.0.0.0:0").map_err(InterError::IOError)?;
    socket.set_nonblocking(true).map_err(InterError::IOError)?;

    let server_addr = addr
      .to_socket_addrs()
      .map_err(InterError::IOError)?
      .next()
      .ok_or(InterError::NoSocketAddr(addr.to_string()))?;

    let stop_flag = Arc::new(AtomicBool::new(false));

    let socket_reader = socket.try_clone().map_err(InterError::IOError)?;
    let server_addr_reader = server_addr.clone();
    let (tx_reader, rx_reader) = mpsc::channel::<InternalSignal>();
    let stop_flag_reader = Arc::clone(&stop_flag);
    let handle_reader = thread::spawn(move || {
      udp_reader(
        socket_reader,
        server_addr_reader,
        tx_reader,
        stop_flag_reader,
      )
    });

    let socket_sender = socket.try_clone().map_err(InterError::IOError)?;
    let server_addr_sender = server_addr.clone();
    let (tx_sender, rx_sender) = mpsc::channel::<InternalSignal>();
    let stop_flag_sender = Arc::clone(&stop_flag);
    let handle_sender = thread::spawn(move || {
      udp_sender(
        socket_sender,
        server_addr_sender,
        rx_sender,
        stop_flag_sender,
      )
    });

    let udp = Self {
      stop_flag,

      handle_reader,
      handle_sender,

      rx_reader,
      tx_sender,

      messages: VecDeque::new(),
    };

    Ok(udp)
  }

  fn stop(self) -> Result<(), InterError> {
    self.stop_flag.store(true, Ordering::Relaxed);

    self
      .handle_reader
      .join()
      .map_err(|e| InterError::ThreadError(format!("{:?}", e)))?;
    self
      .handle_sender
      .join()
      .map_err(|e| InterError::ThreadError(format!("{:?}", e)))?;

    Ok(())
  }

  fn send(&self, msg: String) -> Result<(), InterError> {
    self
      .tx_sender
      .send(msg)
      .map_err(|e| InterError::MPSCSendError(format!("{e:?}")))
  }

  fn fetch(&mut self) -> Result<(), InterError> {
    self.messages.clear();
    loop {
      match self.rx_reader.try_recv() {
        Ok(msg) => {
          self.messages.push_back(msg[..msg.len() - 1].to_string());
        }
        Err(_) => {
          break;
        }
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
