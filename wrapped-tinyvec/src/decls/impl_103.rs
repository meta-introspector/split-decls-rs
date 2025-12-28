macro_rules! deps {
    () => {
        SliceVec!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl < 's , T > Ord for SliceVec < 's , T > where T : Ord , { # [inline] fn cmp (& self , other : & Self) -> core :: cmp :: Ordering { self . as_slice () . cmp (other . as_slice ()) } }
    };
}

impl_103!()