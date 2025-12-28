macro_rules! deps {
    () => {
        Date!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl Date { # [cfg (feature = "serde")] fn type_name () -> & 'static str { "local date" } }
    };
}

impl_9!();