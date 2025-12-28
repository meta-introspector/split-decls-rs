macro_rules! deps {
    () => {
        AstNode!();
        AstPtr!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < N : AstNode > Clone for AstPtr < N > { fn clone (& self) -> AstPtr < N > { * self } }
    };
}

impl_9!()