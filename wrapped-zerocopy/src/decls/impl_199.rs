macro_rules! deps {
    () => {
        SizeError!();
    };
}

macro_rules! impl_199 {
    () => {
        deps!();
        impl < Src : PartialEq , Dst : ? Sized > PartialEq for SizeError < Src , Dst > { # [inline] fn eq (& self , other : & Self) -> bool { self . src == other . src } }
    };
}

impl_199!();