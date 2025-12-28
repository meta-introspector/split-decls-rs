macro_rules! deps {
    () => {
        Opaque!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl std :: fmt :: Debug for Opaque { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{}" , self . 0) } }
    };
}

impl_35!()