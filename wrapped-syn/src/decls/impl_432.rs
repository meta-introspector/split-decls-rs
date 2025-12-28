macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! impl_432 {
    () => {
        deps!();
        impl Display for LitFloat { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { self . repr . token . fmt (formatter) } }
    };
}

impl_432!();