macro_rules! deps {
    () => {
        TopSubtree!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl < S : fmt :: Debug + Copy > fmt :: Debug for TopSubtree < S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& self . view () , f) } }
    };
}

impl_54!();