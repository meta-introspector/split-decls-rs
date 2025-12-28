macro_rules! deps {
    () => {
        TopSubtree!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < S : fmt :: Display + Copy > fmt :: Display for TopSubtree < S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& self . view () , f) } }
    };
}

impl_44!()