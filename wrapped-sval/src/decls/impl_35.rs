macro_rules! deps {
    () => {
        Result!();
        Index!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl fmt :: Display for Index { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Display :: fmt (& self . 0 , f) } }
    };
}

impl_35!()