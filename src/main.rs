extern crate mqtt;
extern crate time;
extern crate threadpool;
extern crate regex;
#[macro_use]
extern crate lazy_static;

mod transport;

use transport::Mqtt;

fn main() {
    Mqtt::new("localhost:1883", "odin")
        .subscribe("/+/switch/+/", Box::new(|(out, msg)| {
            println!("{:?}", msg);
        }))
        .subscribe("/+/odin/+/", Box::new(|(msg, out)| {

        }))
        .run().unwrap();
}