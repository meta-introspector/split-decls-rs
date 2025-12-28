macro_rules! deps {
    () => {
        Fingerprint!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl std :: fmt :: Display for Fingerprint { fn fmt (& self , formatter : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (formatter , "{:x}-{:x}" , self . 0 , self . 1) } }
    };
}

impl_32!()