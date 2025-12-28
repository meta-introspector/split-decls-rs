macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! impl_265 {
    () => {
        deps!();
        impl Default for Span { fn default () -> Self { DUMMY_SP } }
    };
}

impl_265!();