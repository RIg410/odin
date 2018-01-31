use std::io;
use mqtt::packet::PacketError;
use mqtt::packet::Packet;
use mqtt::control::fixed_header::FixedHeaderError;
use mqtt::control::VariableHeaderError;
use mqtt::encodable::StringEncodeError;
use mqtt::topic_name::TopicNameError;
use regex;

#[derive(Debug)]
pub enum TransportError {
    Mqtt(String),
    FixedHeaderError(FixedHeaderError),
    VariableHeaderError(VariableHeaderError),
    PayloadError(String),
    MalformedPacket(String),
    StringEncodeError(StringEncodeError),
    IoError(io::Error),
    TopicNameError(TopicNameError),
    Reqex(regex::Error),
}

impl From<regex::Error> for TransportError {
    fn from(err: regex::Error) -> TransportError {
        TransportError::Reqex(err)
    }
}

impl From<io::Error> for TransportError {
    fn from(err: io::Error) -> TransportError {
        TransportError::IoError(err)
    }
}

impl From<String> for TransportError {
    fn from(err: String) -> TransportError {
        TransportError::Mqtt(err)
    }
}

impl From<TopicNameError> for TransportError {
    fn from(err: TopicNameError) -> TransportError {
        TransportError::TopicNameError(err)
    }
}

impl<'a, T: Packet<'a>> From<PacketError<'a, T>> for TransportError {
    fn from(err: PacketError<'a, T>) -> TransportError {
        match err {
            PacketError::FixedHeaderError(fhe) => TransportError::FixedHeaderError(fhe),
            PacketError::VariableHeaderError(vhe) => TransportError::VariableHeaderError(vhe),
            PacketError::PayloadError(pe) => TransportError::PayloadError(pe.to_string()),
            PacketError::MalformedPacket(err) => TransportError::MalformedPacket(err),
            PacketError::StringEncodeError(enc_err) => TransportError::StringEncodeError(enc_err),
            PacketError::IoError(io_err) => TransportError::IoError(io_err),
            PacketError::TopicNameError(t_err) => TransportError::TopicNameError(t_err),
        }
    }
}