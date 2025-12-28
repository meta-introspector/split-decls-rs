macro_rules! deps {
    () => {
        TinyAsciiStr!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < const N : usize > fmt :: Display for TinyAsciiStr < N > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Display :: fmt (self . as_str () , f) } }
    };
}

impl_10!()