macro_rules! deps {
    () => {
        Error!();
        SerializationStrategy!();
    };
}

macro_rules! impl_294 {
    () => {
        deps!();
        impl serde_core :: ser :: Error for SerializationStrategy { fn custom < T > (_msg : T) -> Self where T : core :: fmt :: Display , { Self :: Unknown } }
    };
}

impl_294!()