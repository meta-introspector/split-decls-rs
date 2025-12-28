macro_rules! deps {
    () => {
        TokenText!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl fmt :: Display for TokenText < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (self . as_str () , f) } }
    };
}

impl_61!()