macro_rules! deps {
    () => {
        DeserializerError!();
        Value!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl std :: error :: Error for DeserializerError { # [inline] fn description (& self) -> & str { "Value deserializer error" } }
    };
}

impl_2!();