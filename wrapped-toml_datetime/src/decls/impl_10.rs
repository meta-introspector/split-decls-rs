macro_rules! deps {
    () => {
        Time!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl Time { # [cfg (feature = "serde")] fn type_name () -> & 'static str { "local time" } }
    };
}

impl_10!()