macro_rules! deps {
    () => {
        AlignmentError!();
    };
}

macro_rules! impl_192 {
    () => {
        deps!();
        impl < Src , Dst : ? Sized > fmt :: Debug for AlignmentError < Src , Dst > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("AlignmentError") . finish () } }
    };
}

impl_192!();