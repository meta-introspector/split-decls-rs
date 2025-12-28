macro_rules! deps {
    () => {
        ArrayVec!();
        Array!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl < A : Array > PartialEq for ArrayVec < A > where A :: Item : PartialEq , { # [inline] fn eq (& self , other : & Self) -> bool { self . as_slice () . eq (other . as_slice ()) } }
    };
}

impl_51!();