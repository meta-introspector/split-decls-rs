macro_rules! deps {
    () => {
        Error!();
        KeyDeserializer!();
        Deserializer!();
    };
}

macro_rules! impl_296 {
    () => {
        deps!();
        impl IntoDeserializer < '_ , Error > for KeyDeserializer { type Deserializer = Self ; fn into_deserializer (self) -> Self :: Deserializer { self } }
    };
}

impl_296!()