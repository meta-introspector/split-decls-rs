macro_rules! deps {
    () => {
        Builder!();
        Sha1!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl Default for Sha1 { fn default () -> Self { Builder :: default () . build () } }
    };
}

impl_80!();