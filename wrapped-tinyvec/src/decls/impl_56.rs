macro_rules! deps {
    () => {
        ArrayVec!();
        Array!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < A : Array > PartialEq < & [A :: Item] > for ArrayVec < A > where A :: Item : PartialEq , { # [inline] fn eq (& self , other : & & [A :: Item]) -> bool { self . as_slice () . eq (* other) } }
    };
}

impl_56!()