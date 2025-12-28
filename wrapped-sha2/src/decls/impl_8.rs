macro_rules! deps {
    () => {
        Sha256VarCore!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl AlgorithmName for Sha256VarCore { # [inline] fn write_alg_name (f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("Sha256") } }
    };
}

impl_8!();