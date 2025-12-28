macro_rules! deps {
    () => {
        SsoHashSet!();
    };
}

macro_rules! impl_463 {
    () => {
        deps!();
        impl < T > Default for SsoHashSet < T > { # [inline] fn default () -> Self { Self :: new () } }
    };
}

impl_463!();