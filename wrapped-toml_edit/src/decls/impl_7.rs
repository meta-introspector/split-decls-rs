macro_rules! deps {
    () => {
        Array!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        # [cfg (feature = "display")] impl std :: fmt :: Display for Array { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { crate :: encode :: encode_array (self , f , None , ("" , "")) } }
    };
}

impl_7!();