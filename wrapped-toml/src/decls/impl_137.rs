macro_rules! deps {
    () => {
        Error!();
        TableDeserializer!();
        Deserializer!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        impl < 'de > IntoDeserializer < 'de , Error > for TableDeserializer < 'de > { type Deserializer = Self ; fn into_deserializer (self) -> Self :: Deserializer { self } }
    };
}

impl_137!()