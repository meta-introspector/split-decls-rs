macro_rules! deps {
    () => {
        Deserializer!();
        Error!();
        ArrayDeserializer!();
    };
}

macro_rules! impl_280 {
    () => {
        deps!();
        impl serde_core :: de :: IntoDeserializer < '_ , Error > for ArrayDeserializer { type Deserializer = Self ; fn into_deserializer (self) -> Self :: Deserializer { self } }
    };
}

impl_280!()