use intercom::client::udp::UdpClient;
use intercom::client::{InterClient, InterClientCommunicator};
use intercom::server::udp::UdpServer;
use intercom::server::{InterServer, InterServerCommunicator};
use intercom::{InterError, InterMessageIncoming, InterMessageOutgoing, InterMessagePrefixed};
use log::warn;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug)]
enum MessageFromClient {
  DoYouWorkProperly,
  AreYouBroken,
  EchoThis(u8),
}

impl InterMessagePrefixed for MessageFromClient {
  fn get_prefix() -> String {
    String::from("simple")
  }
}

impl InterMessageIncoming for MessageFromClient {
  fn from_raw(raw: Vec<&str>) -> Option<Self> {
    match raw.len() {
      1 => match raw.get(0).unwrap() {
        &"doYouWorkProperly" => Some(Self::DoYouWorkProperly),
        &"areYouBroken" => Some(Self::AreYouBroken),
        _ => None,
      },
      2 => match raw.get(0).unwrap() {
        &"echoThis" => match u8::from_str(raw.get(1).unwrap()) {
          Ok(c) => Some(Self::EchoThis(c)),
          Err(e) => {
            warn!("couldn't parse parameter: {e:?}");
            None
          }
        },
        _ => None,
      },
      _ => None,
    }
  }
}

impl InterMessageOutgoing for MessageFromClient {
  fn to_raw(self) -> Result<String, InterError> {
    match self {
      Self::DoYouWorkProperly => Ok(String::from("doYouWorkProperly")),
      Self::AreYouBroken => Ok(String::from("areYouBroken")),
      Self::EchoThis(n) => Ok(format!("echoThis:{}", n)),
    }
  }
}

#[derive(Debug)]
enum MessageFromServer {
  Yes,
  No,
  Number(u8),
}

impl InterMessageIncoming for MessageFromServer {
  fn from_raw(raw: Vec<&str>) -> Option<Self> {
    match raw.len() {
      1 => match raw.get(0).unwrap() {
        &"yes" => Some(Self::Yes),
        &"no" => Some(Self::No),
        _ => None,
      },
      2 => match raw.get(0).unwrap() {
        &"number" => match u8::from_str(raw.get(1).unwrap()) {
          Ok(c) => Some(Self::Number(c)),
          Err(e) => {
            warn!("couldn't parse parameter: {e:?}");
            None
          }
        },
        _ => None,
      },
      _ => None,
    }
  }
}

impl InterMessageOutgoing for MessageFromServer {
  fn to_raw(self) -> Result<String, InterError> {
    Ok(match self {
      Self::Yes => String::from("yes"),
      Self::No => String::from("no"),
      Self::Number(n) => format!("number:{n}"),
    })
  }
}

struct ServerCommunicator {}
impl InterServerCommunicator<UdpServer, MessageFromClient, MessageFromServer>
  for ServerCommunicator
{
}

struct ClientCommunicator {}
impl InterClientCommunicator<UdpClient, MessageFromServer, MessageFromClient>
  for ClientCommunicator
{
}

#[test]
fn simple() {
  let mut server = UdpServer::start("127.0.0.1:21435").unwrap();
  let mut client = UdpClient::start("127.0.0.1:21435").unwrap();

  // do you work properly
  {
    ClientCommunicator::send_message(&client, MessageFromClient::DoYouWorkProperly).unwrap();
    sleep(Duration::from_millis(100));
    server.fetch().unwrap();
    let mut messages = ServerCommunicator::get_messages(&server).unwrap();
    let message = messages.pop_front().unwrap();
    if messages.pop_front().is_some() {
      panic!("found more than one message");
    }
    match message.1 {
      MessageFromClient::DoYouWorkProperly => {
        ServerCommunicator::send_message(&server, message.0, MessageFromServer::Yes).unwrap()
      }
      _ => panic!("incorrect message"),
    }
    sleep(Duration::from_millis(100));
    client.fetch().unwrap();
    let mut messages = ClientCommunicator::get_messages(&client).unwrap();
    let message = messages.pop_front().unwrap();
    if messages.pop_front().is_some() {
      panic!("found more than one message");
    }
    if let MessageFromServer::Yes = message {
    } else {
      panic!("incorrect message")
    }
  }

  // are you broken
  {
    ClientCommunicator::send_message(&client, MessageFromClient::AreYouBroken).unwrap();
    sleep(Duration::from_millis(100));
    server.fetch().unwrap();
    let mut messages = ServerCommunicator::get_messages(&server).unwrap();
    let message = messages.pop_front().unwrap();
    if messages.pop_front().is_some() {
      panic!("found more than one message");
    }
    match message.1 {
      MessageFromClient::AreYouBroken => {
        ServerCommunicator::send_message(&server, message.0, MessageFromServer::No).unwrap()
      }
      _ => panic!("incorrect message"),
    }
    sleep(Duration::from_millis(100));
    client.fetch().unwrap();
    let mut messages = ClientCommunicator::get_messages(&client).unwrap();
    let message = messages.pop_front().unwrap();
    if messages.pop_front().is_some() {
      panic!("found more than one message");
    }
    if let MessageFromServer::No = message {
    } else {
      panic!("incorrect message")
    }
  }

  // // echo
  {
    ClientCommunicator::send_message(&client, MessageFromClient::EchoThis(69)).unwrap();
    sleep(Duration::from_millis(100));
    server.fetch().unwrap();
    let mut messages = ServerCommunicator::get_messages(&server).unwrap();
    let message = messages.pop_front().unwrap();
    if messages.pop_front().is_some() {
      panic!("found more than one message");
    }
    match message.1 {
      MessageFromClient::EchoThis(n) => match n {
        69 => ServerCommunicator::send_message(&server, message.0, MessageFromServer::Number(69))
          .unwrap(),
        _ => panic!("invalid number echoed back"),
      },
      _ => panic!("incorrect message"),
    }
    sleep(Duration::from_millis(100));
    client.fetch().unwrap();
    let mut messages = ClientCommunicator::get_messages(&client).unwrap();
    let message = messages.pop_front().unwrap();
    if messages.pop_front().is_some() {
      panic!("found more than one message");
    }
    if let MessageFromServer::Number(69) = message {
    } else {
      panic!("incorrect message")
    }
  }
}
