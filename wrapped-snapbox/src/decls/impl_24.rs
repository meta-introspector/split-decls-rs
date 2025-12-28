macro_rules! deps {
    () => {
        Result!();
        Backtrace!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        # [cfg (feature = "debug")] impl std :: fmt :: Display for Backtrace { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{:?}" , self . 0) } }
    };
}

impl_24!()