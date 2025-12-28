macro_rules! deps {
    () => {
        ThinVec!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl < A , B > PartialEq < Vec < B > > for ThinVec < A > where A : PartialEq < B > , { # [inline] fn eq (& self , other : & Vec < B >) -> bool { self [..] == other [..] } }
    };
}

impl_35!();