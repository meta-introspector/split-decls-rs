macro_rules! deps {
    () => {
        Error!();
        DeValue!();
        Deserializer!();
        ValueDeserializer!();
    };
}

macro_rules! impl_152 {
    () => {
        deps!();
        impl < 'de > serde_core :: de :: IntoDeserializer < 'de , Error > for Spanned < DeValue < 'de > > { type Deserializer = ValueDeserializer < 'de > ; fn into_deserializer (self) -> Self :: Deserializer { ValueDeserializer :: from (self) } }
    };
}

impl_152!()