macro_rules! deps {
    () => {
        Gid!();
        Result!();
    };
}

macro_rules! impl_1714 {
    () => {
        deps!();
        impl fmt :: UpperHex for Gid { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_1714!()