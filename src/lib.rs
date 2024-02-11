mod util;
pub mod negicon_event;
pub mod spi_protocol;

#[cfg(test)]
mod tests {
    use ux::u7;

    use crate::negicon_event::NegiconEvent;

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
        let deserialized = NegiconEvent::deserialize(serialized);
        assert_eq!(event, deserialized);
    }
}
