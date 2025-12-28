macro_rules! deps {
    () => {
        Error!();
        KeyDeserializer!();
        Deserializer!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl < 'de > IntoDeserializer < 'de , Error > for KeyDeserializer < 'de > { type Deserializer = Self ; fn into_deserializer (self) -> Self :: Deserializer { self } }
    };
}

impl_127!()