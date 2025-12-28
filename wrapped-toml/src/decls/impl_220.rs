macro_rules! deps {
    () => {
        DeFloat!();
    };
}

macro_rules! impl_220 {
    () => {
        deps!();
        impl core :: fmt :: Display for DeFloat < '_ > { fn fmt (& self , formatter : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { self . as_str () . fmt (formatter) ? ; Ok (()) } }
    };
}

impl_220!();