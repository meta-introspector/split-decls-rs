macro_rules! deps {
    () => {
        Sha512VarCore!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl BufferKindUser for Sha512VarCore { type BufferKind = Eager ; }
    };
}

impl_16!();