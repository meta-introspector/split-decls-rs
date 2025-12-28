macro_rules! deps {
    () => {
        TryFromBytes!();
        ValidityError!();
    };
}

macro_rules! impl_210 {
    () => {
        deps!();
        impl < Src , Dst : ? Sized + TryFromBytes > fmt :: Debug for ValidityError < Src , Dst > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("ValidityError") . finish () } }
    };
}

impl_210!();