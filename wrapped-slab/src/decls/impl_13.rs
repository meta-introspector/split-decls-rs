macro_rules! deps {
    () => {
        Slab!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl < T > Default for Slab < T > { fn default () -> Self { Slab :: new () } }
    };
}

impl_13!();