macro_rules! deps {
    () => {
        Sha256VarCore!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl BufferKindUser for Sha256VarCore { type BufferKind = Eager ; }
    };
}

impl_4!()