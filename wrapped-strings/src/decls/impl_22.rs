macro_rules! deps {
    () => {
        HSTRING!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl Default for HSTRING { fn default () -> Self { Self :: new () } }
    };
}

impl_22!();