use crate::InterError;
use log::{error, warn};
use std::collections::{HashMap, VecDeque};
use std::net::{SocketAddr, UdpSocket};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::thread;
use std::thread::JoinHandle;

type ServerInternalSignal = (SocketAddr, String);

pub struct Server {
    stop_flag: Arc<AtomicBool>,

    handle_reader: JoinHandle<()>,
    handle_sender: JoinHandle<()>,

    rx_reader: mpsc::Receiver<ServerInternalSignal>,
    tx_sender: mpsc::Sender<ServerInternalSignal>,

    messages: HashMap<String, VecDeque<(SocketAddr, String)>>,
}

fn udp_reader(
    socket: UdpSocket,
    tx: mpsc::Sender<ServerInternalSignal>,
    stop_flag: Arc<AtomicBool>,
) {
    let mut buf = [0; 1024];
    while !stop_flag.load(Ordering::Relaxed) {
        match socket.recv_from(&mut buf) {
            Ok((len, src)) => {
                let msg = String::from_utf8_lossy(&buf[..len]).to_string();
                match tx.send((src, msg)) {
                    Ok(()) => {}
                    Err(e) => {
                        error!("mpsc send error: {e:?}");
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
    rx: mpsc::Receiver<ServerInternalSignal>,
    stop_flag: Arc<AtomicBool>,
) {
    while !stop_flag.load(Ordering::Relaxed) {
        match rx.try_recv() {
            Ok(v) => {
                let (addr, msg) = v;
                match socket.send_to(msg.as_bytes(), addr) {
                    Ok(_) => {}
                    Err(e) => {
                        error!("couldn't send message on socket: {e:?}");
                    }
                }
            }
            Err(mpsc::TryRecvError::Empty) => {}
            Err(mpsc::TryRecvError::Disconnected) => {
                break;
            }
        }
    }
}

impl Server {
    pub fn start(addr: &str) -> Result<Self, InterError> {
        let socket = UdpSocket::bind(addr).map_err(InterError::IOError)?;
        socket.set_nonblocking(true).map_err(InterError::IOError)?;

        let stop_flag = Arc::new(AtomicBool::new(false));

        let socket_reader = socket.try_clone().map_err(InterError::IOError)?;
        let (tx_reader, rx_reader) = mpsc::channel::<ServerInternalSignal>();
        let stop_flag_reader = Arc::clone(&stop_flag);
        let handle_reader =
            thread::spawn(move || udp_reader(socket_reader, tx_reader, stop_flag_reader));

        let socket_sender = socket.try_clone().map_err(InterError::IOError)?;
        let (tx_sender, rx_sender) = mpsc::channel::<ServerInternalSignal>();
        let stop_flag_sender = Arc::clone(&stop_flag);
        let handle_sender =
            thread::spawn(move || udp_sender(socket_sender, rx_sender, stop_flag_sender));

        let udp = Self {
            stop_flag,

            handle_reader,
            handle_sender,

            rx_reader,
            tx_sender,

            messages: HashMap::new(),
        };

        Ok(udp)
    }

    pub fn stop(self) -> Result<(), InterError> {
        self.stop_flag.store(true, Ordering::Relaxed);

        self.handle_reader
            .join()
            .map_err(|e| InterError::ThreadError(format!("{:?}", e)))?;
        self.handle_sender
            .join()
            .map_err(|e| InterError::ThreadError(format!("{:?}", e)))?;

        Ok(())
    }

    pub fn send(&self, addr: SocketAddr, msg: String) -> Result<(), InterError> {
        self.tx_sender
            .send((addr, msg))
            .map_err(|e| InterError::MPSCSendError(format!("{e:?}")))
    }

    pub fn fetch(&mut self) -> Result<(), InterError> {
        self.messages.clear();
        loop {
            match self.rx_reader.try_recv() {
                Ok((addr, msg)) => match msg.split_once(":") {
                    Some((msg_prefix, msg_content)) => {
                        let msg_prefix_key = msg_prefix.to_string();
                        if !self.messages.contains_key(&msg_prefix_key) {
                            self.messages
                                .insert(msg_prefix_key.clone(), VecDeque::new());
                        }
                        self.messages
                            .get_mut(&msg_prefix_key)
                            .unwrap()
                            .push_back((addr, msg_content.to_string()))
                    }
                    None => {
                        warn!("invalid message received: contained no prefix");
                    }
                },
                Err(_) => {
                    break;
                }
            }
        }
        Ok(())
    }

    pub fn get(&self, prefix: String) -> Option<&VecDeque<(SocketAddr, String)>> {
        self.messages.get(&prefix)
    }
}
