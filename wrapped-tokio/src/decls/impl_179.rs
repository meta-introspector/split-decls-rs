macro_rules! deps {
    () => {
        AtomicU16!();
    };
}

macro_rules! impl_179 {
    () => {
        deps!();
        impl fmt :: Debug for AtomicU16 { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . deref () . fmt (fmt) } }
    };
}

impl_179!();