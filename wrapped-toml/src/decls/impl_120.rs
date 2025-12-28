macro_rules! deps {
    () => {
        Error!();
        Deserializer!();
        ArrayDeserializer!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl < 'de > serde_core :: de :: IntoDeserializer < 'de , Error > for ArrayDeserializer < 'de > { type Deserializer = Self ; fn into_deserializer (self) -> Self :: Deserializer { self } }
    };
}

impl_120!();