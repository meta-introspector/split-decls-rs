macro_rules! deps {
    () => {
        Pu128!();
    };
}

macro_rules! impl_357 {
    () => {
        deps!();
        impl fmt :: UpperHex for Pu128 { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { { self . 0 } . fmt (f) } }
    };
}

impl_357!()