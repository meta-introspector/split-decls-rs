macro_rules! deps {
    () => {
        Key!();
    };
}

macro_rules! impl_139 {
    () => {
        deps!();
        # [cfg (feature = "display")] impl std :: fmt :: Display for Key { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { crate :: encode :: encode_key (self , f , None) } }
    };
}

impl_139!();