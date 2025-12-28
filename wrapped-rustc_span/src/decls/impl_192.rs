macro_rules! deps {
    () => {
        Macros20NormalizedIdent!();
    };
}

macro_rules! impl_192 {
    () => {
        deps!();
        impl fmt :: Debug for Macros20NormalizedIdent { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& self . 0 , f) } }
    };
}

impl_192!();