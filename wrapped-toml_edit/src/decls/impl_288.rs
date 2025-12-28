macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_288 {
    () => {
        deps!();
        impl std :: fmt :: Display for Error { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { self . inner . fmt (f) } }
    };
}

impl_288!()