macro_rules! deps {
    () => {
        InlinedName!();
        Result!();
    };
}

macro_rules! impl_675 {
    () => {
        deps!();
        impl core :: fmt :: Display for InlinedName { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { self . as_str () . fmt (f) } }
    };
}

impl_675!();