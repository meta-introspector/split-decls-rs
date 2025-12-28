macro_rules! deps {
    () => {
        AtomicU32!();
    };
}

macro_rules! impl_188 {
    () => {
        deps!();
        impl fmt :: Debug for AtomicU32 { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . deref () . fmt (fmt) } }
    };
}

impl_188!()