macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_276 {
    () => {
        deps!();
        impl Display for Error { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { Display :: fmt (& self . 0 , f) } }
    };
}

impl_276!();