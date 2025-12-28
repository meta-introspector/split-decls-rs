macro_rules! deps {
    () => {
        SerializerError!();
        Value!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl Error for SerializerError { fn description (& self) -> & str { "Value serializer error" } }
    };
}

impl_25!();