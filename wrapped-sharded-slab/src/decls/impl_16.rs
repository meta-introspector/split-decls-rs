macro_rules! deps {
    () => {
        Slab!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < T > Default for Slab < T > { fn default () -> Self { Self :: new () } }
    };
}

impl_16!()