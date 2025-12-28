macro_rules! deps {
    () => {
        SerializationStrategy!();
        Error!();
    };
}

macro_rules! impl_296 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for SerializationStrategy { }
    };
}

impl_296!();