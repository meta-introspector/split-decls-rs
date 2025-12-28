macro_rules! deps {
    () => {
        Array!();
        TinyVec!();
    };
}

macro_rules! impl_168 {
    () => {
        deps!();
        impl < A : Array > PartialEq < & A > for TinyVec < A > where A :: Item : PartialEq , { # [inline] fn eq (& self , other : & & A) -> bool { self . as_slice () . eq (other . as_slice ()) } }
    };
}

impl_168!()