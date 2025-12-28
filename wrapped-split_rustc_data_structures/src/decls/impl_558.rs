macro_rules! deps {
    () => {
        Svh!();
    };
}

macro_rules! impl_558 {
    () => {
        deps!();
        impl fmt :: Display for Svh { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad (& self . to_hex ()) } }
    };
}

impl_558!()