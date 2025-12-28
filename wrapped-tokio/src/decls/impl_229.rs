macro_rules! deps {
    () => {
        RwLockReadGuard!();
    };
}

macro_rules! impl_229 {
    () => {
        deps!();
        impl < 'a , T : ? Sized + fmt :: Display > fmt :: Display for RwLockReadGuard < 'a , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& self . 1 , f) } }
    };
}

impl_229!();