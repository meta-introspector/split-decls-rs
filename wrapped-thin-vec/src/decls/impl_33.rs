macro_rules! deps {
    () => {
        ThinVec!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl < T > Ord for ThinVec < T > where T : Ord , { # [inline] fn cmp (& self , other : & ThinVec < T >) -> Ordering { self [..] . cmp (& other [..]) } }
    };
}

impl_33!();