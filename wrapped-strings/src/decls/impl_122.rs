macro_rules! deps {
    () => {
        PWSTR!();
    };
}

macro_rules! impl_122 {
    () => {
        deps!();
        impl Default for PWSTR { fn default () -> Self { Self :: null () } }
    };
}

impl_122!()