macro_rules! deps {
    () => {
        Revision!();
    };
}

macro_rules! impl_252 {
    () => {
        deps!();
        impl std :: fmt :: Debug for Revision { fn fmt (& self , fmt : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (fmt , "R{}" , self . generation) } }
    };
}

impl_252!()