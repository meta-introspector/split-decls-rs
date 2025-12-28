macro_rules! deps {
    () => {
        Value!();
        DeserializerError!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl std :: error :: Error for DeserializerError { # [inline] fn description (& self) -> & str { "Value deserializer error" } }
    };
}

impl_2!()