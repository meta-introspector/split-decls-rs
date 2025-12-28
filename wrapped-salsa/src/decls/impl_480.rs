macro_rules! deps {
    () => {
        QueryOrigin!();
    };
}

macro_rules! impl_480 {
    () => {
        deps!();
        impl std :: fmt :: Debug for QueryOrigin { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { self . as_ref () . fmt (f) } }
    };
}

impl_480!()