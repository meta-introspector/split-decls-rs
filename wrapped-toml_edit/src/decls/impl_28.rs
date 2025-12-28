macro_rules! deps {
    () => {
        Document!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl Document < & 'static str > { # [doc = " Creates an empty document"] pub fn new () -> Self { Default :: default () } }
    };
}

impl_28!()