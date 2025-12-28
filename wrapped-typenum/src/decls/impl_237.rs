macro_rules! deps {
    () => {
        InvertedUnsigned!();
        InvertedUTerm!();
    };
}

macro_rules! impl_237 {
    () => {
        deps!();
        impl InvertedUnsigned for InvertedUTerm { # [inline] fn to_u64 () -> u64 { 0 } }
    };
}

impl_237!()