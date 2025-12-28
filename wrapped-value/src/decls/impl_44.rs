macro_rules! deps {
    () => {
        SerializerError!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl ser :: Error for SerializerError { fn custom < T : fmt :: Display > (msg : T) -> SerializerError { SerializerError (msg . to_string ()) } }
    };
}

impl_44!()