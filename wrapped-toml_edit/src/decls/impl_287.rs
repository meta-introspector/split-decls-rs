macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_287 {
    () => {
        deps!();
        impl serde_core :: de :: Error for Error { fn custom < T > (msg : T) -> Self where T : std :: fmt :: Display , { Self :: custom (msg , None) } }
    };
}

impl_287!()