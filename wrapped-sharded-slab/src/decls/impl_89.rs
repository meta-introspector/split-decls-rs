macro_rules! deps {
    () => {
        Generation!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl < C > fmt :: Debug for Generation < C > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("Generation") . field (& self . value) . finish () } }
    };
}

impl_89!();