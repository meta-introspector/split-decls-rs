macro_rules! deps {
    () => {
        AlignmentError!();
    };
}

macro_rules! impl_188 {
    () => {
        deps!();
        impl < Src : PartialEq , Dst : ? Sized > PartialEq for AlignmentError < Src , Dst > { # [inline] fn eq (& self , other : & Self) -> bool { self . src == other . src } }
    };
}

impl_188!();