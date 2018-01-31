use mqtt::packet::PublishPacket;
use mqtt::packet::*;
use mqtt::{Decodable, Encodable, QualityOfService, TopicFilter};
use std::str;
use mqtt::topic_name::*;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use std::sync::Arc;
use std::net::TcpStream;
use std::io::Write;
use std::result;
use threadpool::ThreadPool;
use mqtt::control::variable_header::ConnectReturnCode;
use time;
use regex::Regex;

mod error;

pub use self::error::TransportError;

const KEEP_ALIVE: u16 = 10;
const THREAD_HANDLERS_COUNT: u8 = 2;

pub type Result<T> = result::Result<T, TransportError>;

lazy_static! {
    static ref PING_PAC: Vec<u8> = {
          let pingreq_packet = PingreqPacket::new();
          let mut buf = Vec::new();
          pingreq_packet.encode(&mut buf).unwrap();
          buf
    };
}

pub type MsgHandler = Fn((&mut Sender, Message)) + Send + Sync + 'static;

//TODO rewrite using tokio)
pub struct Mqtt<'a> {
    server_addr: &'a str,
    client_id: &'a str,
    subscribes: Vec<(String, Arc<Box<MsgHandler>>)>,
}

impl<'a> Mqtt<'a> {
    pub fn new(server_addr: &'a str, client_id: &'a str) -> Mqtt<'a> {
        Mqtt { server_addr, client_id, subscribes: Vec::new() }
    }

    pub fn subscribe(mut self, topic: &str, on_msg: Box<MsgHandler>) -> Mqtt<'a> {
        self.subscribes.push((topic.to_owned(), Arc::new(on_msg)));
        self
    }

    pub fn run(self) -> Result<()> {
        let mut stream = TcpStream::connect(self.server_addr)?;
        self.send_connect_packet(&mut stream)?;
        let pac = self.receive_connack_packet(&mut stream).unwrap();
        if pac.connect_return_code() != ConnectReturnCode::ConnectionAccepted {
            return Err(TransportError::Mqtt(format!("Failed to connect to server, return code {:?}", pac.connect_return_code())));
        }

        self.send_subscribe_pac(&mut stream)?;

        self.ping_demon(&mut stream)?;
        let pool = ThreadPool::new(THREAD_HANDLERS_COUNT as usize);

        let pattern = self.make_pattern()?;

        loop {
            let packet = match VariablePacket::decode(&mut stream) {
                Ok(pk) => pk,
                Err(_) => {
                    continue;
                }
            };

            match packet {
                VariablePacket::PublishPacket(publ) => {
                    let index = self.math_index(&pattern, publ.topic_name())?;

                    let handler = self.subscribes[index].1.clone();
                    let mut stream_clone = stream.try_clone().unwrap();

                    pool.execute(move || {
                        let msg = Message::new(publ.topic_name(), &publ.payload()[..]);
                        handler((&mut Sender { tcp_stream: stream_clone }, msg));
                    });
                }
                _ => {}
            }
        }
    }

    fn math_index(&self, pattern: &Vec<Regex>, topic: &str) -> Result<usize> {
        for i in 0..pattern.len() {
            if pattern[i].is_match(topic) {
                return Ok(i);
            }
        }
        Err(TransportError::Mqtt(format!("Topic {} don't match any pattern!", topic)))
    }

    fn make_pattern(&self) -> Result<Vec<Regex>> {
        let mut pattern = Vec::with_capacity(self.subscribes.len());

        for topic in &self.subscribes {
            pattern.push(Regex::new(&topic.0.replace('+', "[0-9A-Za-z_]*"))?);
        }
        Ok(pattern)
    }

    fn send_connect_packet(&self, stream: &mut TcpStream) -> Result<()> {
        let mut conn = ConnectPacket::new("MQTT", self.client_id);
        conn.set_clean_session(true);
        conn.set_keep_alive(KEEP_ALIVE);
        let mut buf = Vec::new();
        conn.encode(&mut buf).unwrap();
        stream.write_all(&buf[..]).unwrap();
        Ok(())
    }

    fn receive_connack_packet(&self, stream: &mut TcpStream) -> Result<ConnackPacket> {
        Ok(ConnackPacket::decode(stream)?)
    }

    fn send_subscribe_pac(&self, stream: &mut TcpStream) -> Result<()> {
        let pac_id: u16 = 10;
        let channel_filters = self.subscribes.iter()
            .map(|t| (TopicFilter::new(t.0.to_string()).unwrap(), QualityOfService::Level0))
            .collect();

        let sub = SubscribePacket::new(pac_id, channel_filters);

        let mut buf = Vec::new();
        sub.encode(&mut buf)?;
        stream.write_all(&buf[..])?;

        loop {
            let packet = match VariablePacket::decode(stream) {
                Ok(pk) => pk,
                Err(_) => {
                    continue;
                }
            };

            if let VariablePacket::SubackPacket(ref ack) = packet {
                if ack.packet_identifier() != pac_id {
                    return Err(TransportError::Mqtt("SUBACK packet identifier not match".to_owned()));
                }
                break;
            }
        }
        Ok(())
    }

    fn ping_demon(&self, stream: &mut TcpStream) -> Result<JoinHandle<u8>> {
        let mut stream_clone = stream.try_clone()?;

        Ok(thread::spawn(move || {
            let mut last_ping_time = 0;
            let mut next_ping_time = last_ping_time + (KEEP_ALIVE as f32 * 0.9) as i64;
            loop {
                let current_timestamp = time::get_time().sec;
                if KEEP_ALIVE > 0 && current_timestamp >= next_ping_time {
                    stream_clone.write_all(&PING_PAC[..]).unwrap();

                    last_ping_time = current_timestamp;
                    next_ping_time = last_ping_time + (KEEP_ALIVE as f32 * 0.9) as i64;
                    thread::sleep(Duration::new((KEEP_ALIVE / 2) as u64, 0));
                }
            }
        }))
    }
}

pub struct Sender {
    tcp_stream: TcpStream,
}

#[derive(Debug)]
pub struct Message<'a> {
    pub topic: &'a str,
    pub payload: Vec<u8>,
}

impl<'a> Message<'a> {
    fn new<P: Into<Vec<u8>>>(topic: &'a str, payload: P) -> Message<'a> {
        Message { topic, payload: payload.into() }
    }
}

impl Sender {
    pub fn send(&mut self, pac: Message) -> Result<()> {
        let pac = PublishPacket::new(
            TopicName::new(pac.topic)?,
            QoSWithPacketIdentifier::Level0,
            pac.payload,
        );

        let mut buf = Vec::new();
        pac.encode(&mut buf)?;
        self.tcp_stream.write_all(&buf[..])?;
        Ok(())
    }
}