use intercom::client::udp::UdpClient;
use intercom::client::{InterClient, InterClientCommunicator};
use intercom::server::udp::UdpServer;
use intercom::server::{InterServer, InterServerCommunicator};
use intercom::{InterMessageIncoming, InterMessageOutgoing, InterMessagePrefixed};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Serialize, Deserialize)]
enum MessageFromClient {
  DoYouWorkProperly,
  AreYouBroken,
  EchoThis(u8),
  EchoThese(u8, i64, u64),
}

impl InterMessagePrefixed for MessageFromClient {
  fn get_prefix() -> String {
    String::from("simple")
  }
}

impl<'de> InterMessageIncoming<'de> for MessageFromClient {}

impl InterMessageOutgoing for MessageFromClient {}

#[derive(Debug, Serialize, Deserialize)]
enum MessageFromServer {
  Yes,
  No,
  Number(u8),
  Numbers(u8, i64, u64),
}

impl<'de> InterMessageIncoming<'de> for MessageFromServer {}

impl InterMessageOutgoing for MessageFromServer {}

// struct ServerCommunicator {}
// impl InterServerCommunicator<UdpServer, MessageFromClient, MessageFromServer>
//   for ServerCommunicator
// {
// }

// struct ClientCommunicator {}
// impl InterClientCommunicator<UdpClient, MessageFromServer, MessageFromClient>
//   for ClientCommunicator
// {
// }

type ServerCommunicator<'de> =
  InterServerCommunicator<'de, UdpServer, MessageFromClient, MessageFromServer>;
type ClientCommunicator<'de> =
  InterClientCommunicator<'de, UdpClient, MessageFromServer, MessageFromClient>;

#[tokio::test]
async fn simple() {
  let _ = env_logger::builder().is_test(true).try_init();

  let mut server = UdpServer::start("127.0.0.1:21435").await.unwrap();
  let mut client = UdpClient::start("127.0.0.1:21435").await.unwrap();

  // do you work properly
  {
    ClientCommunicator::send_message(&client, MessageFromClient::DoYouWorkProperly)
      .await
      .unwrap();
    sleep(Duration::from_millis(100)).await;
    server.fetch().await.unwrap();
    let mut messages = ServerCommunicator::get_messages(&server).unwrap();
    let message = messages.pop_front().unwrap();
    if messages.pop_front().is_some() {
      panic!("found more than one message");
    }
    match message.1 {
      MessageFromClient::DoYouWorkProperly => {
        ServerCommunicator::send_message(&server, message.0, MessageFromServer::Yes)
          .await
          .unwrap()
      }
      _ => panic!("incorrect message"),
    }
    sleep(Duration::from_millis(100)).await;
    client.fetch().await.unwrap();
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
    ClientCommunicator::send_message(&client, MessageFromClient::AreYouBroken)
      .await
      .unwrap();
    sleep(Duration::from_millis(100)).await;
    server.fetch().await.unwrap();
    let mut messages = ServerCommunicator::get_messages(&server).unwrap();
    let message = messages.pop_front().unwrap();
    if messages.pop_front().is_some() {
      panic!("found more than one message");
    }
    match message.1 {
      MessageFromClient::AreYouBroken => {
        ServerCommunicator::send_message(&server, message.0, MessageFromServer::No)
          .await
          .unwrap()
      }
      _ => panic!("incorrect message"),
    }
    sleep(Duration::from_millis(100)).await;
    client.fetch().await.unwrap();
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
    ClientCommunicator::send_message(&client, MessageFromClient::EchoThis(69))
      .await
      .unwrap();
    sleep(Duration::from_millis(100)).await;
    server.fetch().await.unwrap();
    let mut messages = ServerCommunicator::get_messages(&server).unwrap();
    let message = messages.pop_front().unwrap();
    if messages.pop_front().is_some() {
      panic!("found more than one message");
    }
    match message.1 {
      MessageFromClient::EchoThis(n) => match n {
        69 => ServerCommunicator::send_message(&server, message.0, MessageFromServer::Number(69))
          .await
          .unwrap(),
        _ => panic!("invalid number echoed back"),
      },
      _ => panic!("incorrect message"),
    }
    sleep(Duration::from_millis(100)).await;
    client.fetch().await.unwrap();
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

  {
    ClientCommunicator::send_message(&client, MessageFromClient::EchoThese(69, -12, 420))
      .await
      .unwrap();
    sleep(Duration::from_millis(100)).await;
    server.fetch().await.unwrap();
    let mut messages = ServerCommunicator::get_messages(&server).unwrap();
    let message = messages.pop_front().unwrap();
    if messages.pop_front().is_some() {
      panic!("found more than one message");
    }
    match message.1 {
      MessageFromClient::EchoThese(a, b, c) => match (a, b, c) {
        (69, -12, 420) => ServerCommunicator::send_message(
          &server,
          message.0,
          MessageFromServer::Numbers(69, -12, 420),
        )
        .await
        .unwrap(),
        _ => panic!("invalid numbers echoed back"),
      },
      _ => panic!("incorrect message"),
    }
    sleep(Duration::from_millis(100)).await;
    client.fetch().await.unwrap();
    let mut messages = ClientCommunicator::get_messages(&client).unwrap();
    let message = messages.pop_front().unwrap();
    if messages.pop_front().is_some() {
      panic!("found more than one message");
    }
    if let MessageFromServer::Numbers(69, -12, 420) = message {
    } else {
      panic!("incorrect message")
    }
  }
}
