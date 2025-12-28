macro_rules! deps {
    () => {
        RustcPatCtxt!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl < 'p , 'tcx : 'p > fmt :: Debug for RustcPatCtxt < 'p , 'tcx > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("RustcPatCtxt") . finish () } }
    };
}

impl_79!();