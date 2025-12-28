macro_rules! deps {
    () => {
        SsoHashSet!();
    };
}

macro_rules! impl_468 {
    () => {
        deps!();
        impl < T > fmt :: Debug for SsoHashSet < T > where T : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_set () . entries (self . iter ()) . finish () } }
    };
}

impl_468!();