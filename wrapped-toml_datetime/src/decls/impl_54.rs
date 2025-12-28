macro_rules! deps {
    () => {
        SerializerError!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl serde_core :: ser :: Error for SerializerError { fn custom < T > (_msg : T) -> Self where T : core :: fmt :: Display , { Self :: InvalidProtocol } }
    };
}

impl_54!()