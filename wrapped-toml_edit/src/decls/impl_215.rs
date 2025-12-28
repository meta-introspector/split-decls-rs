macro_rules! deps {
    () => {
        Formatted!();
        ValueRepr!();
    };
}

macro_rules! impl_215 {
    () => {
        deps!();
        # [cfg (feature = "display")] impl < T > std :: fmt :: Display for Formatted < T > where T : ValueRepr , { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { crate :: encode :: encode_formatted (self , f , None , ("" , "")) } }
    };
}

impl_215!();