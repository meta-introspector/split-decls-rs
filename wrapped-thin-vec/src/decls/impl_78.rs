macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl < T : fmt :: Debug > fmt :: Debug for Drain < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("Drain") . field (& self . iter . as_slice ()) . finish () } }
    };
}

impl_78!();