macro_rules! deps {
    () => {
        AtomicUsize!();
    };
}

macro_rules! impl_201 {
    () => {
        deps!();
        impl fmt :: Debug for AtomicUsize { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* * self) . fmt (fmt) } }
    };
}

impl_201!()