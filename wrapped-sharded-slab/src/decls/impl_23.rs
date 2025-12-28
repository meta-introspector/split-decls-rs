macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < T , C > fmt :: Debug for Entry < '_ , T , C > where T : fmt :: Debug , C : cfg :: Config , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (self . value () , f) } }
    };
}

impl_23!()