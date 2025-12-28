macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_289 {
    () => {
        deps!();
        impl std :: fmt :: Debug for Error { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { self . inner . fmt (f) } }
    };
}

impl_289!();