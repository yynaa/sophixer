use log::warn;
use std::net::UdpSocket;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;
use tin_intercom::{
    InterCommunicator, InterError, InterMessageIncoming, InterMessageOutgoing, Server,
};

#[derive(Debug)]
enum MessageIncoming {
    DoYouWorkProperly,
    AreYouBroken,
    EchoThis(u8),
}

impl InterMessageIncoming for MessageIncoming {
    fn get_prefix() -> String {
        "simple".to_string()
    }

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

enum MessageOutgoing {
    Yes,
    No,
    Number(u8),
}

impl InterMessageOutgoing for MessageOutgoing {
    fn to_raw(self) -> Result<String, InterError> {
        Ok(match self {
            MessageOutgoing::Yes => String::from("yes"),
            MessageOutgoing::No => String::from("no"),
            MessageOutgoing::Number(n) => format!("number:{n}"),
        })
    }
}

struct SimpleCommunicator {}
impl InterCommunicator<MessageIncoming, MessageOutgoing> for SimpleCommunicator {}

#[test]
fn simple() {
    let mut server = Server::start("127.0.0.1:21435").unwrap();
    let client = UdpSocket::bind("0.0.0.0:0").unwrap();

    // do you work properly
    {
        client
            .send_to("simple:doYouWorkProperly".as_bytes(), "127.0.0.1:21435")
            .unwrap();
        sleep(Duration::from_millis(100));
        server.fetch().unwrap();
        let mut messages = SimpleCommunicator::get_messages(&server).unwrap();
        let message = messages.pop_front().unwrap();
        if messages.pop_front().is_some() {
            panic!("found more than one message");
        }
        match message.1 {
            MessageIncoming::DoYouWorkProperly => {
                SimpleCommunicator::send_message(&server, message.0, MessageOutgoing::Yes).unwrap()
            }
            _ => panic!("incorrect message"),
        }
        sleep(Duration::from_millis(100));
        let mut buf = [0; 128];
        let (len, _) = client.recv_from(&mut buf).unwrap();
        let string = str::from_utf8(&buf[..len]).unwrap();
        if string != "yes" {
            panic!("incorrect message");
        }
    }

    // are you broken
    {
        client
            .send_to("simple:areYouBroken".as_bytes(), "127.0.0.1:21435")
            .unwrap();
        sleep(Duration::from_millis(100));
        server.fetch().unwrap();
        let mut messages = SimpleCommunicator::get_messages(&server).unwrap();
        let message = messages.pop_front().unwrap();
        if messages.pop_front().is_some() {
            panic!("found more than one message");
        }
        match message.1 {
            MessageIncoming::AreYouBroken => {
                SimpleCommunicator::send_message(&server, message.0, MessageOutgoing::No).unwrap()
            }
            _ => panic!("incorrect message"),
        }
        sleep(Duration::from_millis(100));
        let mut buf = [0; 128];
        let (len, _) = client.recv_from(&mut buf).unwrap();
        let string = str::from_utf8(&buf[..len]).unwrap();
        if string != "no" {
            panic!("incorrect message");
        }
    }

    // echo
    {
        client
            .send_to("simple:echoThis:69".as_bytes(), "127.0.0.1:21435")
            .unwrap();
        sleep(Duration::from_millis(100));
        server.fetch().unwrap();
        let mut messages = SimpleCommunicator::get_messages(&server).unwrap();
        let message = messages.pop_front().unwrap();
        if messages.pop_front().is_some() {
            panic!("found more than one message");
        }
        match message.1 {
            MessageIncoming::EchoThis(69) => {
                SimpleCommunicator::send_message(&server, message.0, MessageOutgoing::Number(69))
                    .unwrap()
            }
            _ => panic!("incorrect message"),
        }
        sleep(Duration::from_millis(100));
        let mut buf = [0; 128];
        let (len, _) = client.recv_from(&mut buf).unwrap();
        let string = str::from_utf8(&buf[..len]).unwrap();
        if string != "number:69" {
            panic!("incorrect message");
        }
    }
}
