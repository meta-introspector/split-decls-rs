macro_rules! deps {
    () => {
        SliceVec!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl < 's , T > PartialEq < & [T] > for SliceVec < 's , T > where T : PartialEq , { # [inline] fn eq (& self , other : & & [T]) -> bool { self . as_slice () . eq (* other) } }
    };
}

impl_104!()