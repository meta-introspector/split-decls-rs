macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! impl_429 {
    () => {
        deps!();
        impl Display for LitInt { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { self . repr . token . fmt (formatter) } }
    };
}

impl_429!()