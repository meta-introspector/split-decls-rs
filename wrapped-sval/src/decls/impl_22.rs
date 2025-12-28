macro_rules! deps {
    () => {
        Label!();
        Result!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < 'a > fmt :: Debug for Label < 'a > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_tuple ("Label") . field (& self . as_str ()) . finish () } }
    };
}

impl_22!();