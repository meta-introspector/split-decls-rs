macro_rules! deps {
    () => {
        Slab!();
    };
}

macro_rules! impl_184 {
    () => {
        deps!();
        impl < T > Default for Slab < T > { fn default () -> Self { Self :: new () } }
    };
}

impl_184!()