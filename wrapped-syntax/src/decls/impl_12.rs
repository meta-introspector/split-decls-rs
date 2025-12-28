macro_rules! deps {
    () => {
        AstNode!();
        AstPtr!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl < N : AstNode > Hash for AstPtr < N > { fn hash < H : Hasher > (& self , state : & mut H) { self . raw . hash (state) ; } }
    };
}

impl_12!()