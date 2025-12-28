macro_rules! deps {
    () => {
        Array!();
        TinyVec!();
    };
}

macro_rules! impl_169 {
    () => {
        deps!();
        impl < A : Array > PartialEq < & [A :: Item] > for TinyVec < A > where A :: Item : PartialEq , { # [inline] fn eq (& self , other : & & [A :: Item]) -> bool { self . as_slice () . eq (* other) } }
    };
}

impl_169!()