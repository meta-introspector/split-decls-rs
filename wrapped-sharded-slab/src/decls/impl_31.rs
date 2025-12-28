macro_rules! deps {
    () => {
        Config!();
        Clear!();
        OwnedRef!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl < T , C > fmt :: Debug for OwnedRef < T , C > where T : fmt :: Debug + Clear + Default , C : cfg :: Config , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (self . value () , f) } }
    };
}

impl_31!();