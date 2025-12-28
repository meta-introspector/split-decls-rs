macro_rules! deps {
    () => {
        ConstValue!();
        DeserializerError!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl IntoDeserializer < '_ , DeserializerError > for ConstValue { type Deserializer = Self ; fn into_deserializer (self) -> Self :: Deserializer { self } }
    };
}

impl_11!();