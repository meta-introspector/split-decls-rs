macro_rules! deps {
    () => {
        Sharded!();
    };
}

macro_rules! impl_389 {
    () => {
        deps!();
        impl < T : Default > Default for Sharded < T > { # [inline] fn default () -> Self { Self :: new (T :: default) } }
    };
}

impl_389!();