macro_rules! deps {
    () => {
        PCWSTR!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        impl Default for PCWSTR { fn default () -> Self { Self :: null () } }
    };
}

impl_113!()