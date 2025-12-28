macro_rules! deps {
    () => {
        OwnedEntry!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl < T , C > fmt :: Debug for OwnedEntry < T , C > where T : fmt :: Debug , C : cfg :: Config , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (self . value () , f) } }
    };
}

impl_30!()