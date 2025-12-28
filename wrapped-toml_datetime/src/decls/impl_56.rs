macro_rules! deps {
    () => {
        SerializerError!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for SerializerError { }
    };
}

impl_56!()