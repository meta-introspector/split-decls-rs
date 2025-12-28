macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_348 {
    () => {
        deps!();
        impl serde_core :: ser :: Error for Error { fn custom < T > (msg : T) -> Self where T : std :: fmt :: Display , { Self :: custom (msg) } }
    };
}

impl_348!()