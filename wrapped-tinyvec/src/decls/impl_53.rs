macro_rules! deps {
    () => {
        ArrayVec!();
        Array!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl < A : Array > PartialOrd for ArrayVec < A > where A :: Item : PartialOrd , { # [inline] fn partial_cmp (& self , other : & Self) -> Option < core :: cmp :: Ordering > { self . as_slice () . partial_cmp (other . as_slice ()) } }
    };
}

impl_53!()