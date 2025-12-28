macro_rules! deps {
    () => {
        Sha512VarCore!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl OutputSizeUser for Sha512VarCore { type OutputSize = U64 ; }
    };
}

impl_18!()