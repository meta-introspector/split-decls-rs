macro_rules! deps {
    () => {
        DelayedSet!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < T > Default for DelayedSet < T > { fn default () -> Self { DelayedSet { cache : Default :: default () , count : 0 } } }
    };
}

impl_8!()