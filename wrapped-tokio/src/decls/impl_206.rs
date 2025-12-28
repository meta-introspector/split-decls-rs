macro_rules! deps {
    () => {
        Barrier!();
    };
}

macro_rules! impl_206 {
    () => {
        deps!();
        impl fmt :: Debug for Barrier { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Barrier") . finish_non_exhaustive () } }
    };
}

impl_206!()