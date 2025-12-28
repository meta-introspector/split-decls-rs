macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_164 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl serde_core :: de :: Error for Error { fn custom < T > (msg : T) -> Self where T : core :: fmt :: Display , { Self :: custom (msg . to_string () , None) } }
    };
}

impl_164!();