macro_rules! deps {
    () => {
        Spanned!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < T : core :: fmt :: Display > core :: fmt :: Display for Spanned < T > { fn fmt (& self , fmt : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { self . get_ref () . fmt (fmt) } }
    };
}

impl_9!();