macro_rules! deps {
    () => {
        SizableArc!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl < T : core :: fmt :: Debug , SC : ShouldCountInner > core :: fmt :: Debug for SizableArc < T , SC > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_76!()