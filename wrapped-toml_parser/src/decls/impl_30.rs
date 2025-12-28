macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl core :: fmt :: Debug for Span { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { (self . start .. self . end) . fmt (f) } }
    };
}

impl_30!()