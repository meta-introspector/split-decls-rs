macro_rules! deps {
    () => {
        AstPtr!();
        AstNode!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < N : AstNode > PartialEq for AstPtr < N > { fn eq (& self , other : & AstPtr < N >) -> bool { self . raw == other . raw } }
    };
}

impl_11!()