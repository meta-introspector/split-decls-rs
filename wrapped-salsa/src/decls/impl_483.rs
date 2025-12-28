macro_rules! deps {
    () => {
        QueryEdge!();
    };
}

macro_rules! impl_483 {
    () => {
        deps!();
        impl std :: fmt :: Debug for QueryEdge { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { self . kind () . fmt (f) } }
    };
}

impl_483!()