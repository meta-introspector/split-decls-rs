macro_rules! deps {
    () => {
        HasTypeFlagsVisitor!();
    };
}

macro_rules! impl_503 {
    () => {
        deps!();
        impl std :: fmt :: Debug for HasTypeFlagsVisitor { fn fmt (& self , fmt : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { self . flags . fmt (fmt) } }
    };
}

impl_503!();