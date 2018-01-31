extern crate mqtt;
extern crate time;
extern crate threadpool;
#[macro_use]
extern crate lazy_static;

mod transport;

use transport::Transport;

use std::str;
use std::thread;
use mqtt::packet::PublishPacket;
use mqtt::packet::Packet;
use mqtt::packet::QoSWithPacketIdentifier;
use mqtt::TopicName;

fn main() {
    let tr = Transport::new("localhost:1883", "odin", vec!("odin"));

    tr.bind(|(out, pcg)| {
        let msg = match str::from_utf8(&pcg.payload()[..]) {
            Ok(msg) => msg,
            Err(err) => {
                ""
            }
        };

        out.send(PublishPacket::new(TopicName::new("tor").unwrap(), QoSWithPacketIdentifier::Level0, "ku tor! I am Odin"));
        println!("PUBLISH ({}): {}: {:?}", pcg.topic_name(), msg, thread::current().id());
    });
}