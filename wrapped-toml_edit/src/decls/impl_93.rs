macro_rules! deps {
    () => {
        InlineTable!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        # [cfg (feature = "display")] impl std :: fmt :: Display for InlineTable { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { crate :: encode :: encode_table (self , f , None , ("" , "")) } }
    };
}

impl_93!()