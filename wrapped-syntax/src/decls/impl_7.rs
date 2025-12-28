macro_rules! deps {
    () => {
        AstNode!();
        AstPtr!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl < N : AstNode > std :: fmt :: Debug for AstPtr < N > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_tuple ("AstPtr") . field (& self . raw) . finish () } }
    };
}

impl_7!()