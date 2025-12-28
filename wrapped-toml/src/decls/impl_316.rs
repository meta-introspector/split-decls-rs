macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_316 {
    () => {
        deps!();
        impl serde_core :: ser :: Error for Error { fn custom < T > (msg : T) -> Self where T : core :: fmt :: Display , { Self :: new (msg) } }
    };
}

impl_316!()