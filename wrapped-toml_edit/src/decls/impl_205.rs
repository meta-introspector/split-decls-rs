macro_rules! deps {
    () => {
        RawStringInner!();
        RawString!();
    };
}

macro_rules! impl_205 {
    () => {
        deps!();
        impl Default for RawString { fn default () -> Self { Self (RawStringInner :: Empty) } }
    };
}

impl_205!();