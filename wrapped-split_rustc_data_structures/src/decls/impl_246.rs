macro_rules! deps {
    () => {
        Interned!();
    };
}

macro_rules! impl_246 {
    () => {
        deps!();
        impl < T : Debug > Debug for Interned < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_246!()