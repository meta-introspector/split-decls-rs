macro_rules! deps {
    () => {
        Result!();
        ErrorMessage!();
    };
}

macro_rules! impl_181 {
    () => {
        deps!();
        impl Debug for ErrorMessage { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { Debug :: fmt (& self . message , formatter) } }
    };
}

impl_181!();