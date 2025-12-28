macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl < T > fmt :: Debug for Iter < '_ , T > where T : fmt :: Debug , { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_struct ("Iter") . field ("remaining" , & self . len) . finish () } }
    };
}

impl_33!()