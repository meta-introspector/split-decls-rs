macro_rules! deps {
    () => {
        TinyVec!();
        Array!();
    };
}

macro_rules! impl_164 {
    () => {
        deps!();
        impl < A : Array > PartialEq for TinyVec < A > where A :: Item : PartialEq , { # [inline] fn eq (& self , other : & Self) -> bool { self . as_slice () . eq (other . as_slice ()) } }
    };
}

impl_164!();