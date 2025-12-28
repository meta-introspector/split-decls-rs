macro_rules! deps {
    () => {
        ConstValue!();
        SerializerError!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl Error for SerializerError { fn description (& self) -> & str { "ConstValue serializer error" } }
    };
}

impl_43!()