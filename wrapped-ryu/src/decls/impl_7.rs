macro_rules! deps {
    () => {
        Buffer!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl Default for Buffer { # [inline] # [cfg_attr (feature = "no-panic" , no_panic)] fn default () -> Self { Buffer :: new () } }
    };
}

impl_7!()