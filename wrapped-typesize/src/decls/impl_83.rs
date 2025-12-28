macro_rules! deps {
    () => {
        SizableRc!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl < T : core :: fmt :: Debug , SC : ShouldCountInner > core :: fmt :: Debug for SizableRc < T , SC > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_83!();