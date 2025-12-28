macro_rules! deps {
    () => {
        MovePath!();
        Formatter!();
    };
}

macro_rules! impl_188 {
    () => {
        deps!();
        impl < 'tcx > fmt :: Display for MovePath < 'tcx > { fn fmt (& self , w : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (w , "{:?}" , self . place) } }
    };
}

impl_188!()