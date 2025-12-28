macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < T > fmt :: Debug for Drain < '_ , T > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_struct ("Drain") . finish () } }
    };
}

impl_29!()