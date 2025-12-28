macro_rules! deps {
    () => {
        SerializationStrategy!();
    };
}

macro_rules! impl_297 {
    () => {
        deps!();
        # [cfg (not (feature = "std"))] impl serde_core :: de :: StdError for SerializationStrategy { }
    };
}

impl_297!()