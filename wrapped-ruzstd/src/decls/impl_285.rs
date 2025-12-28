macro_rules! deps {
    () => {
        LiteralsSection!();
    };
}

macro_rules! impl_285 {
    () => {
        deps!();
        impl Default for LiteralsSection { fn default () -> Self { Self :: new () } }
    };
}

impl_285!()