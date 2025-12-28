macro_rules! deps {
    () => {
        Sha512VarCore!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl AlgorithmName for Sha512VarCore { # [inline] fn write_alg_name (f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("Sha512") } }
    };
}

impl_20!();