macro_rules! deps {
    () => {
        TryInitError!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        impl fmt :: Display for TryInitError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { # [cfg (feature = "std")] { fmt :: Display :: fmt (& self . inner , f) } # [cfg (not (feature = "std"))] { f . write_str ("failed to set global default subscriber") } } }
    };
}

impl_151!()