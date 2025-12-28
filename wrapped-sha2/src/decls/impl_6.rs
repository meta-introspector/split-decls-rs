macro_rules! deps {
    () => {
        Sha256VarCore!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl OutputSizeUser for Sha256VarCore { type OutputSize = U32 ; }
    };
}

impl_6!();