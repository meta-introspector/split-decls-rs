macro_rules! deps {
    () => {
        Result!();
        Gid!();
    };
}

macro_rules! impl_1712 {
    () => {
        deps!();
        impl fmt :: Octal for Gid { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_1712!();