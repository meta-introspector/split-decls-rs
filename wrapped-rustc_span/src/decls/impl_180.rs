macro_rules! deps {
    () => {
        Ident!();
    };
}

macro_rules! impl_180 {
    () => {
        deps!();
        impl fmt :: Debug for Ident { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (self , f) ? ; fmt :: Debug :: fmt (& self . span . ctxt () , f) } }
    };
}

impl_180!()