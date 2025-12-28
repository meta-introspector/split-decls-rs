macro_rules! deps {
    () => {
        DeserializerError!();
        Value!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl Error for DeserializerError { fn description (& self) -> & str { "Value deserializer error" } }
    };
}

impl_6!()