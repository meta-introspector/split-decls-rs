macro_rules! deps {
    () => {
        Deserializer!();
        Error!();
        DeTable!();
    };
}

macro_rules! impl_160 {
    () => {
        deps!();
        impl < 'de > serde_core :: de :: IntoDeserializer < 'de , Error > for Spanned < DeTable < 'de > > { type Deserializer = Deserializer < 'de > ; fn into_deserializer (self) -> Self :: Deserializer { Deserializer :: from (self) } }
    };
}

impl_160!()