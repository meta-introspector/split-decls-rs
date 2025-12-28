macro_rules! deps {
    () => {
        AtomicIterationCount!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl std :: fmt :: Display for AtomicIterationCount { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { self . load () . fmt (f) } }
    };
}

impl_50!()