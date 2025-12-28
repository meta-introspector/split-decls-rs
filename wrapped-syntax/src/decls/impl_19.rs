macro_rules! deps {
    () => {
        SyntaxError!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl fmt :: Display for SyntaxError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_19!()