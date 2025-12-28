macro_rules! deps {
    () => {
        TokenText!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl fmt :: Debug for TokenText < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (self . as_str () , f) } }
    };
}

impl_62!();