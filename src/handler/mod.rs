mod switch;
mod leaks;
mod dimmer;

pub use self::dimmer::Dimmer;
pub use self::leaks::LeakHandler;
pub use self::switch::{SwitchHandler, SwitchHolder};

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_location() {
        assert_eq!(Some("corridor_1"), parse_id("/switch/corridor_1"));
        assert_eq!(Some("corridor"), parse_id("/switch/corridor"));
    }
}
