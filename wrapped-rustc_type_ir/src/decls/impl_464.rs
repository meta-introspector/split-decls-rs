macro_rules! deps {
    () => {
        Interner!();
        UnsafeBinderInner!();
    };
}

macro_rules! impl_464 {
    () => {
        deps!();
        impl < I : Interner > fmt :: Debug for UnsafeBinderInner < I > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_464!();