macro_rules! deps {
    () => {
        Result!();
        Error!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("signature error") } }
    };
}

impl_13!()