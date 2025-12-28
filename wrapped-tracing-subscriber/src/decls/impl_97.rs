macro_rules! deps {
    () => {
        Context!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl < S > Clone for Context < '_ , S > { # [inline] fn clone (& self) -> Self { let subscriber = self . subscriber . as_ref () . copied () ; Context { subscriber , # [cfg (all (feature = "registry" , feature = "std"))] filter : self . filter , } } }
    };
}

impl_97!();