macro_rules! deps {
    () => {
        SliceVec!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl < 's , T > PartialEq for SliceVec < 's , T > where T : PartialEq , { # [inline] fn eq (& self , other : & Self) -> bool { self . as_slice () . eq (other . as_slice ()) } }
    };
}

impl_100!();