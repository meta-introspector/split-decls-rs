macro_rules! deps {
    () => {
        Error!();
        DocumentMut!();
        Deserializer!();
    };
}

macro_rules! impl_334 {
    () => {
        deps!();
        impl serde_core :: de :: IntoDeserializer < '_ , Error > for crate :: DocumentMut { type Deserializer = Deserializer ; fn into_deserializer (self) -> Self :: Deserializer { Deserializer :: from (self) } }
    };
}

impl_334!();