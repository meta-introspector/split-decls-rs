macro_rules! deps {
    () => {
        Sha1!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl core :: fmt :: Debug for Sha1 { # [inline] fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> Result < () , core :: fmt :: Error > { f . write_str ("Sha1CollisionDetection { .. }") } }
    };
}

impl_11!()