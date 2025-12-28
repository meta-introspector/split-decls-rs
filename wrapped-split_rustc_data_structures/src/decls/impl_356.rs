macro_rules! deps {
    () => {
        Pu128!();
    };
}

macro_rules! impl_356 {
    () => {
        deps!();
        impl fmt :: Display for Pu128 { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { { self . 0 } . fmt (f) } }
    };
}

impl_356!();