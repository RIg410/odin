use super::MessageHandler;
use transport::{Message, MqPublisher};
pub struct Odin {

}

impl MessageHandler for Odin {
    fn handel(&self, msg: &Message, publisher: &mut MqPublisher) -> Result<Option<String>, Option<String>> {
        unimplemented!()
    }
}