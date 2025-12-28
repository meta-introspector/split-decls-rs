macro_rules! deps {
    () => {
        RevealedTy!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl < 'tcx > fmt :: Display for RevealedTy < 'tcx > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 . fmt (fmt) } }
    };
}

impl_73!()