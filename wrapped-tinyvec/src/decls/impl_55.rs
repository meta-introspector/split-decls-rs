macro_rules! deps {
    () => {
        Array!();
        ArrayVec!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl < A : Array > PartialEq < & A > for ArrayVec < A > where A :: Item : PartialEq , { # [inline] fn eq (& self , other : & & A) -> bool { self . as_slice () . eq (other . as_slice ()) } }
    };
}

impl_55!();