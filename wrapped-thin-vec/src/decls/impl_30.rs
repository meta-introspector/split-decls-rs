macro_rules! deps {
    () => {
        ThinVec!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl < T : fmt :: Debug > fmt :: Debug for ThinVec < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
    };
}

impl_30!();