macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_320 {
    () => {
        deps!();
        # [cfg (not (feature = "std"))] impl serde_core :: de :: StdError for Error { }
    };
}

impl_320!();