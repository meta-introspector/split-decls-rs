macro_rules! deps {
    () => {
        Error!();
        Deserializer!();
        Value!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl IntoDeserializer < '_ , crate :: de :: Error > for Value { type Deserializer = Self ; fn into_deserializer (self) -> Self { self } }
    };
}

impl_98!();