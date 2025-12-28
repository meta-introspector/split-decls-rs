macro_rules! deps {
    () => {
        KeyMut!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        # [cfg (feature = "display")] impl std :: fmt :: Display for KeyMut < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { std :: fmt :: Display :: fmt (& self . key , f) } }
    };
}

impl_151!()