macro_rules! deps {
    () => {
        ThinVec!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl < 'a , A , B > PartialEq < & 'a [B] > for ThinVec < A > where A : PartialEq < B > , { # [inline] fn eq (& self , other : & & 'a [B]) -> bool { self [..] == other [..] } }
    };
}

impl_37!()