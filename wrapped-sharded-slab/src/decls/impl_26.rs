macro_rules! deps {
    () => {
        RefMut!();
        Config!();
        Clear!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < T , C > fmt :: Debug for RefMut < '_ , T , C > where T : fmt :: Debug + Clear + Default , C : cfg :: Config , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (self . value () , f) } }
    };
}

impl_26!()