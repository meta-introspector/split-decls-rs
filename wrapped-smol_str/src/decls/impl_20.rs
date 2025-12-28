macro_rules! deps {
    () => {
        SmolStr!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl fmt :: Display for SmolStr { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (self . as_str () , f) } }
    };
}

impl_20!()