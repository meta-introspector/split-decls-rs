macro_rules! deps {
    () => {
        Error!();
        Result!();
    };
}

macro_rules! impl_182 {
    () => {
        deps!();
        impl Display for Error { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str (& self . messages [0] . message) } }
    };
}

impl_182!()