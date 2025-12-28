macro_rules! deps {
    () => {
        Opaque!();
    };
}

macro_rules! impl_500 {
    () => {
        deps!();
        impl std :: fmt :: Display for Opaque { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{}" , self . 0) } }
    };
}

impl_500!();