macro_rules! deps {
    () => {
        IterationCount!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl std :: fmt :: Display for IterationCount { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_47!()