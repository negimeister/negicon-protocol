#![no_std]

pub mod negicon_event;
pub mod ringbuf;
pub mod util;

pub use negicon_event::*;
pub use util::*;

#[cfg(test)]
mod tests {
    use ux::{i24};

    use crate::negicon_event::{InvalidMessage, NegiconEvent};

    use super::*;

    #[test]
    fn event_de_serialize() {
        let event = NegiconEvent::new(0x1234, 0x12u8, i24::new(-1), 0x9a);
        let serialized = event.serialize();
        let deserialized = NegiconEvent::deserialize(&serialized);
        assert_eq!(event, deserialized.unwrap());
    }

    #[test]
    fn detect_corruption() {
        let event = NegiconEvent::new(0x1234, 0x12u8, i24::new(0x5678), 0x9a);
        let mut serialized = event.serialize();
        serialized[5] = serialized[5].reverse_bits();
        let deserialized = NegiconEvent::deserialize(&serialized);
        assert_eq!(InvalidMessage::CrcError, deserialized.unwrap_err());
    }

    #[test]
    fn ping() {
        let event = NegiconEvent::ping(0x1234, NegiconModuleType::Controller);
        let serialized = event.serialize();
        let deserialized = NegiconEvent::deserialize(&serialized);
        assert_eq!(event, deserialized.unwrap());
        assert!(event.is_ping());
    }

    #[test]
    fn empty() {
        let event = NegiconEvent::empty(0x1234, NegiconModuleType::AnalogPaddle, 1);
        let serialized = event.serialize();
        let deserialized = NegiconEvent::deserialize(&serialized);
        assert_eq!(event, deserialized.unwrap());
        
    }
}
