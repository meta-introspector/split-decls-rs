macro_rules! deps {
    () => {
        Flags!();
    };
}

macro_rules! impl_233 {
    () => {
        deps!();
        impl Default for Flags { fn default () -> Self { Self :: SANE } }
    };
}

impl_233!();