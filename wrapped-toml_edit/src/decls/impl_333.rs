macro_rules! deps {
    () => {
        Deserializer!();
        Error!();
    };
}

macro_rules! impl_333 {
    () => {
        deps!();
        impl serde_core :: de :: IntoDeserializer < '_ , Error > for Deserializer { type Deserializer = Self ; fn into_deserializer (self) -> Self :: Deserializer { self } }
    };
}

impl_333!();