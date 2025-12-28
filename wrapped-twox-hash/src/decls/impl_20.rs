macro_rules! deps {
    () => {
        Hasher!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl Default for Hasher { fn default () -> Self { Self :: with_seed (0) } }
    };
}

impl_20!()