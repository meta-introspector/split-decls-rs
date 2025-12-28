macro_rules! deps {
    () => {
        Array!();
        TinyVec!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        impl < A : Array > Ord for TinyVec < A > where A :: Item : Ord , { # [inline] fn cmp (& self , other : & Self) -> core :: cmp :: Ordering { self . as_slice () . cmp (other . as_slice ()) } }
    };
}

impl_167!();