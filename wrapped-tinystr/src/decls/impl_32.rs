macro_rules! deps {
    () => {
        UnvalidatedTinyAsciiStr!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < const N : usize > fmt :: Debug for UnvalidatedTinyAsciiStr < N > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . try_into_tinystr () { Ok (s) => fmt :: Debug :: fmt (& s , f) , Err (_) => fmt :: Debug :: fmt (& self . 0 , f) , } } }
    };
}

impl_32!();