macro_rules! deps {
    () => {
        FatalError!();
    };
}

macro_rules! impl_227 {
    () => {
        deps!();
        impl std :: fmt :: Display for FatalError { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "fatal error") } }
    };
}

impl_227!();