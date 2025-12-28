macro_rules! deps {
    () => {
        SliceVec!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl < 's , T > PartialOrd for SliceVec < 's , T > where T : PartialOrd , { # [inline] fn partial_cmp (& self , other : & Self) -> Option < core :: cmp :: Ordering > { self . as_slice () . partial_cmp (other . as_slice ()) } }
    };
}

impl_102!()