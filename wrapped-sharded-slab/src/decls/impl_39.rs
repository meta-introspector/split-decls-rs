macro_rules! deps {
    () => {
        OwnedRefMut!();
        Clear!();
        Config!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < T , C > fmt :: Debug for OwnedRefMut < T , C > where T : fmt :: Debug + Clear + Default , C : cfg :: Config , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (self . value () , f) } }
    };
}

impl_39!()