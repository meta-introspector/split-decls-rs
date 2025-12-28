macro_rules! deps {
    () => {
        Deserializer!();
        Error!();
        Document!();
    };
}

macro_rules! impl_335 {
    () => {
        deps!();
        impl serde_core :: de :: IntoDeserializer < '_ , Error > for crate :: Document < String > { type Deserializer = Deserializer ; fn into_deserializer (self) -> Self :: Deserializer { Deserializer :: from (self) } }
    };
}

impl_335!();