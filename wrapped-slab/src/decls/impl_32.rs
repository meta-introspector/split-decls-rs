macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < T > fmt :: Debug for IntoIter < T > where T : fmt :: Debug , { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_struct ("IntoIter") . field ("remaining" , & self . len) . finish () } }
    };
}

impl_32!();