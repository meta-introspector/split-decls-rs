macro_rules! deps {
    () => {
        RwLockWriteGuard!();
    };
}

macro_rules! impl_230 {
    () => {
        deps!();
        impl < 'a , T : ? Sized + fmt :: Display > fmt :: Display for RwLockWriteGuard < 'a , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& self . 1 , f) } }
    };
}

impl_230!();