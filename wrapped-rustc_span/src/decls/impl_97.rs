macro_rules! deps {
    () => {
        DefPathHash!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl Default for DefPathHash { fn default () -> Self { DefPathHash (Fingerprint :: ZERO) } }
    };
}

impl_97!()