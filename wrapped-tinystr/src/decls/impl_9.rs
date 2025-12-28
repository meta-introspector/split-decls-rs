macro_rules! deps {
    () => {
        TinyAsciiStr!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < const N : usize > fmt :: Debug for TinyAsciiStr < N > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Debug :: fmt (self . as_str () , f) } }
    };
}

impl_9!();