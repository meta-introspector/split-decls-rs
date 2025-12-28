macro_rules! deps {
    () => {
        ArrayVec!();
        Array!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl < A : Array > Ord for ArrayVec < A > where A :: Item : Ord , { # [inline] fn cmp (& self , other : & Self) -> core :: cmp :: Ordering { self . as_slice () . cmp (other . as_slice ()) } }
    };
}

impl_54!()