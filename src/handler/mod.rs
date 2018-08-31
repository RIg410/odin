mod switch;
mod leaks;

pub use self::leaks::LeakHandler;
pub use self::switch::SwitchHandler;
use super::transport::MqPublisher;
use super::transport::Message;
use chrono::prelude::*;

const LOG_TOPIC: &str = "/odin/log/heimdallr/";

pub fn parse_id(topic: &str) -> Option<&str> {
    let mut from = -2;
    for (i, c) in topic.char_indices() {
        if from == -2 && c == '/' {
            from = -1;
            continue;
        }
        if from == -1 && c == '/' {
            from = (i as i32) + 1;
            break;
        }
    }
    sub_str(topic, from, topic.len() as i32)
}

#[inline]
fn sub_str(s: &str, from: i32, to: i32) -> Option<&str> {
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
    fn test_parse_location() {
        assert_eq!(Some("corridor_1"), parse_id("/switch/corridor_1"));
        assert_eq!(Some("corridor"), parse_id("/switch/corridor"));
    }
}
