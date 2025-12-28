macro_rules! deps {
    () => {
        ValueDeserializer!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < 'de , E > de :: IntoDeserializer < 'de , E > for ValueDeserializer < E > where E : de :: Error , { type Deserializer = Self ; fn into_deserializer (self) -> Self :: Deserializer { self } }
    };
}

impl_16!()