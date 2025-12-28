macro_rules! deps {
    () => {
        ValueDeserializer!();
        Deserializer!();
        Error!();
    };
}

macro_rules! impl_319 {
    () => {
        deps!();
        impl serde_core :: de :: IntoDeserializer < '_ , Error > for ValueDeserializer { type Deserializer = Self ; fn into_deserializer (self) -> Self :: Deserializer { self } }
    };
}

impl_319!()