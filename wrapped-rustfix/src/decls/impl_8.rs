macro_rules! deps {
    () => {
        LineRange!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl std :: fmt :: Display for LineRange { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{}-{}" , self . start , self . end) } }
    };
}

impl_8!()