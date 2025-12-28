macro_rules! deps {
    () => {
        JoinedArgs!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl std :: fmt :: Display for JoinedArgs { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { self . to_string () . fmt (f) } }
    };
}

impl_22!();