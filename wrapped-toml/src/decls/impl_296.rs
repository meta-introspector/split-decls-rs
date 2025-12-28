macro_rules! deps {
    () => {
        Error!();
        SerializationStrategy!();
    };
}

macro_rules! impl_296 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for SerializationStrategy { }
    };
}

impl_296!()