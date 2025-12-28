macro_rules! deps {
    () => {
        Value!();
        ValueDeserializer!();
        Error!();
        Deserializer!();
        Item!();
    };
}

macro_rules! impl_320 {
    () => {
        deps!();
        impl serde_core :: de :: IntoDeserializer < '_ , Error > for crate :: Value { type Deserializer = ValueDeserializer ; fn into_deserializer (self) -> Self :: Deserializer { ValueDeserializer :: new (crate :: Item :: Value (self)) } }
    };
}

impl_320!()