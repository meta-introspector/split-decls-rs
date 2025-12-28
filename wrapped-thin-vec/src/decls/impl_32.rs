macro_rules! deps {
    () => {
        ThinVec!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < T > PartialOrd for ThinVec < T > where T : PartialOrd , { # [inline] fn partial_cmp (& self , other : & ThinVec < T >) -> Option < Ordering > { self [..] . partial_cmp (& other [..]) } }
    };
}

impl_32!()