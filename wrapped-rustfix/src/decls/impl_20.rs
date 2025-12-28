macro_rules! deps {
    () => {
        LinePosition!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl std :: fmt :: Display for LinePosition { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{}:{}" , self . line , self . column) } }
    };
}

impl_20!();