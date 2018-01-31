extern crate mqtt;
extern crate time;
extern crate threadpool;
extern crate regex;
#[macro_use]
extern crate lazy_static;

mod transport;

use transport::{Mqtt, Message};

fn main() {
    Mqtt::new("localhost:1883", "odin")
        .subscribe("/+/switch/+/", |(out, msg)| {
            println!("{:?}", msg);
            out.send(Message::new("tor", "val")).unwrap();
        })
        .subscribe("/+/odin/+/", |(out, msg)| {
            println!("{:?}", msg);
            out.send(Message::new("tor", "val_1")).unwrap();
        })
        .run().unwrap();
}