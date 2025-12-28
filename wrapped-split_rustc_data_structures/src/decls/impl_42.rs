macro_rules! deps {
    () => {
        PackedFingerprint!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl std :: fmt :: Display for PackedFingerprint { # [inline] fn fmt (& self , formatter : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { let copy = self . 0 ; copy . fmt (formatter) } }
    };
}

impl_42!()