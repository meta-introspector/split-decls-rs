macro_rules! deps {
    () => {
        Sha1!();
        Builder!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl Default for Sha1 { fn default () -> Self { Builder :: default () . build () } }
    };
}

impl_7!()