macro_rules! deps {
    () => {
        RevealedTy!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl < 'tcx > fmt :: Debug for RevealedTy < 'tcx > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 . fmt (fmt) } }
    };
}

impl_74!()