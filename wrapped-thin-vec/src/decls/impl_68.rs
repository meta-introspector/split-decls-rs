macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl < T : fmt :: Debug > fmt :: Debug for IntoIter < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("IntoIter") . field (& self . as_slice ()) . finish () } }
    };
}

impl_68!();