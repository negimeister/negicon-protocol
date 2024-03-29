#![no_std]

pub mod negicon_event;
pub mod ringbuf;
pub mod util;

pub use negicon_event::*;
pub use util::*;

#[cfg(test)]
mod tests {
    use ux::u7;

    use crate::negicon_event::{InvalidMessage, NegiconEvent};

    use super::*;

    #[test]
    fn event_de_serialize() {
        let event = NegiconEvent::new(
            negicon_event::NegiconEventType::Input,
            0x1234,
            u7::new(0x12),
            0x5678,
            0x9a,
            0xbc,
        );
        let serialized = event.serialize();
        let deserialized = NegiconEvent::deserialize(&serialized);
        assert_eq!(event, deserialized.unwrap());
    }

    #[test]
    fn detect_corruption() {
        let event = NegiconEvent::new(
            negicon_event::NegiconEventType::Input,
            0x1234,
            u7::new(0x12),
            0x5678,
            0x9a,
            0xbc,
        );
        let mut serialized = event.serialize();
        serialized[5] = serialized[5].reverse_bits();
        let deserialized = NegiconEvent::deserialize(&serialized);
        assert_eq!(InvalidMessage::CrcError, deserialized.unwrap_err());
    }

    #[test]
    fn ping() {
        let event = NegiconEvent::ping(0x1234, NegiconModuleType::Controller, 0x56);
        let serialized = event.serialize();
        let deserialized = NegiconEvent::deserialize(&serialized);
        assert_eq!(event, deserialized.unwrap());
        assert!(event.is_ping());
    }
}
