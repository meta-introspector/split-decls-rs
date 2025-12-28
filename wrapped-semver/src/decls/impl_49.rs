macro_rules! deps {
    () => {
        Identifier!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl Default for Identifier { fn default () -> Self { Identifier :: empty () } }
    };
}

impl_49!()