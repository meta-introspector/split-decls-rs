macro_rules! deps {
    () => {
        Array!();
        TinyVec!();
    };
}

macro_rules! impl_166 {
    () => {
        deps!();
        impl < A : Array > PartialOrd for TinyVec < A > where A :: Item : PartialOrd , { # [inline] fn partial_cmp (& self , other : & Self) -> Option < core :: cmp :: Ordering > { self . as_slice () . partial_cmp (other . as_slice ()) } }
    };
}

impl_166!();