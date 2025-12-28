macro_rules! deps {
    () => {
        Error!();
        Deserializer!();
        ValueDeserializer!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        impl < 'de > serde_core :: de :: IntoDeserializer < 'de , Error > for ValueDeserializer < 'de > { type Deserializer = Self ; fn into_deserializer (self) -> Self :: Deserializer { self } }
    };
}

impl_151!();