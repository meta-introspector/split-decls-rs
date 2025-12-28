macro_rules! deps {
    () => {
        DeserializerError!();
        Value!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl < 'de > de :: IntoDeserializer < 'de , DeserializerError > for Value { type Deserializer = Value ; fn into_deserializer (self) -> Value { self } }
    };
}

impl_12!();