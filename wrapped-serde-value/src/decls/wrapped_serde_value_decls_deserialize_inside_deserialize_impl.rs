use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[test]
fn deserialize_inside_deserialize_impl() {
    #[derive(Debug, PartialEq, Eq)]
    enum Event {
        Added(u32),
        Error(u8),
    }
    impl<'de> serde::Deserialize<'de> for Event {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            #[derive(Deserialize)]
            struct RawEvent {
                kind: String,
                object: Value,
            }
            let raw_event = RawEvent::deserialize(deserializer)?;
            let object_deserializer = ValueDeserializer::new(raw_event.object);
            Ok(
                match &*raw_event.kind {
                    "ADDED" => Event::Added(<_>::deserialize(object_deserializer)?),
                    "ERROR" => Event::Error(<_>::deserialize(object_deserializer)?),
                    kind => {
                        return Err(
                            serde::de::Error::unknown_variant(kind, &["ADDED", "ERROR"]),
                        );
                    }
                },
            )
        }
    }
    let input = Value::Map(
        vec![
            (Value::String("kind".to_owned()), Value::String("ADDED".to_owned()),),
            (Value::String("object".to_owned()), Value::U32(5)),
        ]
            .into_iter()
            .collect(),
    );
    let event = Event::deserialize(input).expect("could not deserialize ADDED event");
    assert_eq!(event, Event::Added(5));
    let input = Value::Map(
        vec![
            (Value::String("kind".to_owned()), Value::String("ERROR".to_owned()),),
            (Value::String("object".to_owned()), Value::U8(5)),
        ]
            .into_iter()
            .collect(),
    );
    let event = Event::deserialize(input).expect("could not deserialize ERROR event");
    assert_eq!(event, Event::Error(5));
    let input = Value::Map(
        vec![
            (Value::String("kind".to_owned()), Value::String("ADDED".to_owned()),),
            (Value::String("object".to_owned()), Value::Unit),
        ]
            .into_iter()
            .collect(),
    );
    let _ = Event::deserialize(input)
        .expect_err("expected deserializing bad ADDED event to fail");
}
