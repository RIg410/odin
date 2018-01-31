use mqtt::packet::PublishPacket;
use mqtt::packet::*;
use mqtt::{Decodable, Encodable, QualityOfService, TopicFilter};
use std::str;
use std::ops::FnOnce;
use mqtt::topic_name::*;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use std::sync::Arc;
use std::net::TcpStream;
use std::io::Write;
use threadpool::ThreadPool;
use mqtt::control::variable_header::ConnectReturnCode;
use time;

const KEEP_ALIVE: u16 = 10;
const THREAD_HANDLERS_COUNT: u8 = 2;

lazy_static! {
    static ref PING_PAC: Vec<u8> = {
          let pingreq_packet = PingreqPacket::new();
          let mut buf = Vec::new();
          pingreq_packet.encode(&mut buf).unwrap();
          buf
    };
}

pub struct Transport<'a> {
    server_addr: &'a str,
    client_id: &'a str,
    topics: Vec<&'a str>,
}

impl<'a> Transport<'a> {
    pub fn new(server_addr: &'a str, client_id: &'a str, topics: Vec<&'a str>) -> Transport<'a> {
        Transport { server_addr, client_id, topics }
    }

    pub fn bind<F>(self, on_msg: F) -> Result<(), ()>
        where F: Fn((&mut Output, PublishPacket)) + Send + Sync + 'static {
        let mut stream = TcpStream::connect(self.server_addr).unwrap();

        self.send_connect_packet(&mut stream);
        let pac = self.receive_connack_packet(&mut stream).unwrap();
        if pac.connect_return_code() != ConnectReturnCode::ConnectionAccepted {
            panic!("Failed to connect to server, return code {:?}", pac.connect_return_code());
        }

        self.subscribe(&mut stream).unwrap();

        let ping_demon = self.ping_demon(&mut stream);

        let pool = ThreadPool::new(THREAD_HANDLERS_COUNT as usize);

        let hndl = Arc::new(on_msg);
        loop {
            let packet = match VariablePacket::decode(&mut stream) {
                Ok(pk) => pk,
                Err(err) => {
                    continue;
                }
            };

            match packet {
                VariablePacket::PublishPacket(publ) => {
                    let mut stream_clone = stream.try_clone().unwrap();
                    let hndl = hndl.clone();
                    pool.execute(move|| {
                        hndl((&mut Output { tcp_stream: stream_clone }, publ));
                    });
                }
                _ => {}
            }
        }
    }

    fn ping_demon(&self, stream: &mut TcpStream) -> JoinHandle<u8> {
        let mut stream_clone = stream.try_clone().unwrap();

        thread::spawn(move || {
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
        })
    }

    fn subscribe(&self, stream: &mut TcpStream) -> Result<(), ()> {
        let pac_id: u16 = 10;
        let channel_filters = self.topics.iter()
            .map(|t| (TopicFilter::new(t.to_string()).unwrap(), QualityOfService::Level0))
            .collect();

        let sub = SubscribePacket::new(pac_id, channel_filters);

        let mut buf = Vec::new();
        sub.encode(&mut buf).unwrap();
        stream.write_all(&buf[..]).unwrap();

        loop {
            let packet = match VariablePacket::decode(stream) {
                Ok(pk) => pk,
                Err(err) => {
                    continue;
                }
            };

            if let VariablePacket::SubackPacket(ref ack) = packet {
                if ack.packet_identifier() != pac_id {
                    panic!("SUBACK packet identifier not match");
                }

                println!("Subscribed!");
                break;
            }
        }
        Ok(())
    }

    fn receive_connack_packet(&self, stream: &mut TcpStream) -> Result<ConnackPacket, ()> {
        Ok(ConnackPacket::decode(stream).unwrap())
    }

    fn send_connect_packet(&self, stream: &mut TcpStream) -> Result<(), ()> {
        let mut conn = ConnectPacket::new("MQTT", self.client_id);
        conn.set_clean_session(true);
        conn.set_keep_alive(KEEP_ALIVE);
        let mut buf = Vec::new();
        conn.encode(&mut buf).unwrap();
        stream.write_all(&buf[..]).unwrap();
        Ok(())
    }
}

pub struct Output {
    tcp_stream: TcpStream,
}

impl Output {
    pub fn send(&mut self, pac: PublishPacket) -> Result<(), ()> {
        let mut buf = Vec::new();
        pac.encode(&mut buf).unwrap();
        self.tcp_stream.write_all(&buf[..]).unwrap();
        Ok(())
    }
}