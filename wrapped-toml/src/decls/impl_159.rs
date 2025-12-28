macro_rules! deps {
    () => {
        Deserializer!();
        Error!();
    };
}

macro_rules! impl_159 {
    () => {
        deps!();
        impl < 'de > serde_core :: de :: IntoDeserializer < 'de , Error > for Deserializer < 'de > { type Deserializer = Self ; fn into_deserializer (self) -> Self :: Deserializer { self } }
    };
}

impl_159!()