macro_rules! deps {
    () => {
        ErrorKind!();
    };
}

macro_rules! impl_420 {
    () => {
        deps!();
        impl core :: fmt :: Display for ErrorKind { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . write_str (self . as_str ()) } }
    };
}

impl_420!();