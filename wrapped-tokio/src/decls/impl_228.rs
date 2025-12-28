macro_rules! deps {
    () => {
        MutexGuard!();
    };
}

macro_rules! impl_228 {
    () => {
        deps!();
        impl < 'a , T : ? Sized + fmt :: Display > fmt :: Display for MutexGuard < 'a , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& self . 1 , f) } }
    };
}

impl_228!();