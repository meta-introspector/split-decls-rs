macro_rules! deps {
    () => {
        SerializerError!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl ser :: Error for SerializerError { fn custom < T : fmt :: Display > (msg : T) -> SerializerError { SerializerError :: Custom (msg . to_string ()) } }
    };
}

impl_26!();