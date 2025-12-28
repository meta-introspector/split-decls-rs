macro_rules! deps {
    () => {
        DisplayValue!();
    };
}

macro_rules! impl_154 {
    () => {
        deps!();
        impl < T : fmt :: Display > fmt :: Debug for DisplayValue < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (self , f) } }
    };
}

impl_154!()