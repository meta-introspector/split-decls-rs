macro_rules! deps {
    () => {
        Sha1Core!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl AlgorithmName for Sha1Core { fn write_alg_name (f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("Sha1") } }
    };
}

impl_11!()