macro_rules! deps {
    () => {
        ThinVec!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl < A , B > PartialEq < [B] > for ThinVec < A > where A : PartialEq < B > , { # [inline] fn eq (& self , other : & [B]) -> bool { self [..] == other [..] } }
    };
}

impl_36!();