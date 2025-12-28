macro_rules! deps {
    () => {
        Macros20NormalizedIdent!();
    };
}

macro_rules! impl_193 {
    () => {
        deps!();
        impl fmt :: Display for Macros20NormalizedIdent { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& self . 0 , f) } }
    };
}

impl_193!();