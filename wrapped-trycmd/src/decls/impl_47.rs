macro_rules! deps {
    () => {
        BinRegistry!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl Default for BinRegistry { fn default () -> Self { Self :: new () } }
    };
}

impl_47!();