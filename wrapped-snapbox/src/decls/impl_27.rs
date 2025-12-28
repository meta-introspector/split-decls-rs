macro_rules! deps {
    () => {
        Backtrace!();
        Result!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        # [cfg (not (feature = "debug"))] impl std :: fmt :: Display for Backtrace { fn fmt (& self , _ : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { Ok (()) } }
    };
}

impl_27!();