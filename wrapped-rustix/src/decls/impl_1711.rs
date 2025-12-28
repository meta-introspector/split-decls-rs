macro_rules! deps {
    () => {
        Gid!();
        Result!();
    };
}

macro_rules! impl_1711 {
    () => {
        deps!();
        impl fmt :: Binary for Gid { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_1711!()