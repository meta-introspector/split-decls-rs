macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl < T > fmt :: Debug for Drain < '_ , T > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_struct ("Drain") . finish () } }
    };
}

impl_35!()