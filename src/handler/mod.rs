mod switch;
mod odin;

pub use self::switch::Switch;
pub use self::odin::Odin;
use super::transport::MqPublisher;
use super::transport::Message;
use chrono::prelude::*;
use std::fmt::Debug;

const LOG_TOPIC: &str = "/odin/log/heimdallr/";

pub fn parse_receiver<'a>(topic: &'a str) -> Option<&'a str> {
    let mut to = -1;
    let mut from = -1;
    for (i, c) in topic.char_indices().rev() {
        if to == -1 && c == '/' {
            to = i as i32;
            continue;
        }
        if from == -1 && c == '/' {
            from = (i as i32) + 1;
            break;
        }
    }

    sub_str(topic, from, to)
}

pub fn parse_type<'a>(topic: &'a str) -> Option<&'a str> {
    let mut to = -1;
    let mut from = -2;
    for (i, c) in topic.char_indices() {
        if from == -2 && c == '/' {
            from = -1;
            continue;
        }
        if from == -1 && c == '/' {
            from = (i as i32) + 1;
            continue;
        }
        if to == -1 && c == '/' {
            to = i as i32;
            break;
        }
    }

    sub_str(topic, from, to)
}

pub fn parse_sender<'a>(topic: &'a str) -> Option<&'a str> {
    let mut to = -1;
    let mut from = -1;
    for (i, c) in topic.char_indices() {
        if from == -1 && c == '/' {
            from = (i as i32) + 1;
            continue;
        }
        if to == -1 && c == '/' {
            to = i as i32;
            break;
        }
    }

    sub_str(topic, from, to)
}

fn sub_str<'a>(s: &'a str, from: i32, to: i32) -> Option<&'a str> {
    if from < 0 || to < 0 || from > to {
        None
    } else {
        Some(&s[from as usize..to as usize])
    }
}


pub trait MessageHandler {
    fn handel(&self, msg: &Message, publisher: &mut MqPublisher) -> Result<Option<String>, Option<String>>;

    fn on_message(&self, msg: &Message, publisher: &mut MqPublisher) {
        let res = self.handel(msg, publisher);
        match res {
            Ok(Some(res)) => send_log(publisher, res, msg, LogType::info),
            Err(Some(res)) => send_log(publisher, res, msg, LogType::error),
            _ => {}
        }
    }
}

#[derive(Debug)]
enum LogType {
    info,
    error,
}

fn send_log(publisher: &mut MqPublisher, log: String, msg: &Message, log_lvl: LogType) {
    let log_line = format!("{:?}-{:?}-{}-{:?}", Local::now(), log_lvl, msg.topic, log);
    let res = publisher.publish(Message::new(LOG_TOPIC, log_line));
    if res.is_err() {
        // TODO
    }
}

mod test {
    use super::*;

    #[test]
    fn parse_receiver_test() {
        assert_eq!(Some("id_1"), parse_receiver("/i1/switch/id_1/"));
        assert_eq!(Some(""), parse_receiver("/i1/switch//"));
        assert_eq!(None, parse_receiver("roomswitch/"))
    }

    #[test]
    fn test_parse_location() {
        assert_eq!(Some("r_1"), parse_sender("/r_1/switch/id_1/"));
        assert_eq!(Some("r"), parse_sender("/r/switch//"));
        assert_eq!(None, parse_sender("roomswitch/"))
    }

    #[test]
    fn test_parse_type() {
        assert_eq!(Some("switch"), parse_type("/r_1/switch/id_1/"));
        assert_eq!(Some("log"), parse_type("/r/log//"));
        assert_eq!(None, parse_type("roomswitch/"))
    }
}
