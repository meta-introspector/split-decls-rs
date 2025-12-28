macro_rules! deps {
    () => {
        ThinVec!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl < A , B > PartialEq < ThinVec < B > > for ThinVec < A > where A : PartialEq < B > , { # [inline] fn eq (& self , other : & ThinVec < B >) -> bool { self [..] == other [..] } }
    };
}

impl_34!()