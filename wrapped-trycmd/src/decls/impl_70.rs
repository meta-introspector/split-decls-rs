macro_rules! deps {
    () => {
        Stdio!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl std :: fmt :: Display for Stdio { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { self . as_str () . fmt (f) } }
    };
}

impl_70!()