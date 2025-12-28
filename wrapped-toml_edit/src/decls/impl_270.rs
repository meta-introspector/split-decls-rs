macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_270 {
    () => {
        deps!();
        # [cfg (feature = "display")] impl std :: fmt :: Display for Value { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { crate :: encode :: encode_value (self , f , None , ("" , "")) } }
    };
}

impl_270!()