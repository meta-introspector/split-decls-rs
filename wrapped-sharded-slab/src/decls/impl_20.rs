macro_rules! deps {
    () => {
        Ref!();
        Clear!();
        Config!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < T , C > fmt :: Debug for Ref < '_ , T , C > where T : fmt :: Debug + Clear + Default , C : cfg :: Config , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (self . value () , f) } }
    };
}

impl_20!();