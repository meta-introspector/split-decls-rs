macro_rules! deps {
    () => {
        Deserializer!();
        TableDeserializer!();
        Error!();
    };
}

macro_rules! impl_306 {
    () => {
        deps!();
        impl IntoDeserializer < '_ , Error > for TableDeserializer { type Deserializer = Self ; fn into_deserializer (self) -> Self :: Deserializer { self } }
    };
}

impl_306!()