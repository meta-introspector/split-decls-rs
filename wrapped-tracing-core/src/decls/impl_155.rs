macro_rules! deps {
    () => {
        DisplayValue!();
    };
}

macro_rules! impl_155 {
    () => {
        deps!();
        impl < T : fmt :: Display > fmt :: Display for DisplayValue < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_155!();