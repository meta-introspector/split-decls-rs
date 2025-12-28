macro_rules! deps {
    () => {
        Deserializer!();
        Error!();
        Table!();
    };
}

macro_rules! impl_378 {
    () => {
        deps!();
        impl de :: IntoDeserializer < '_ , crate :: de :: Error > for Table { type Deserializer = Self ; fn into_deserializer (self) -> Self { self } }
    };
}

impl_378!()