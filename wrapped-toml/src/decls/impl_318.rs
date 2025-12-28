macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_318 {
    () => {
        deps!();
        impl core :: fmt :: Debug for Error { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { self . inner . fmt (f) } }
    };
}

impl_318!();