macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_168 {
    () => {
        deps!();
        # [cfg (not (feature = "std"))] # [cfg (feature = "serde")] impl serde_core :: de :: StdError for Error { }
    };
}

impl_168!()