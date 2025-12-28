macro_rules! deps {
    () => {
        PSTR!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl Default for PSTR { fn default () -> Self { Self :: null () } }
    };
}

impl_118!()