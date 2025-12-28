macro_rules! deps {
    () => {
        NewSpan!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl fmt :: Display for NewSpan { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "a new span{}" , self . span . metadata) ? ; if ! self . fields . is_empty () { write ! (f , " with {}" , self . fields) ? ; } Ok (()) } }
    };
}

impl_58!();