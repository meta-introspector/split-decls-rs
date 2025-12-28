macro_rules! deps {
    () => {
        Error!();
        Repr!();
    };
}

macro_rules! impl_220 {
    () => {
        deps!();
        impl std :: fmt :: Debug for Repr { # [inline] fn fmt (& self , formatter : & mut std :: fmt :: Formatter < '_ >) -> Result < () , std :: fmt :: Error > { self . raw_value . fmt (formatter) } }
    };
}

impl_220!();