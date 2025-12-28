macro_rules! deps {
    () => {
        Sha1Core!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl BufferKindUser for Sha1Core { type BufferKind = Eager ; }
    };
}

impl_5!();