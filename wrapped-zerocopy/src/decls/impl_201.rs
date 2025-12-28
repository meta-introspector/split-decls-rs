macro_rules! deps {
    () => {
        SizeError!();
    };
}

macro_rules! impl_201 {
    () => {
        deps!();
        impl < Src , Dst : ? Sized > fmt :: Debug for SizeError < Src , Dst > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("SizeError") . finish () } }
    };
}

impl_201!();