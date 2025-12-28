macro_rules! deps {
    () => {
        AstNode!();
        AstPtr!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < N : AstNode > Copy for AstPtr < N > { }
    };
}

impl_8!()