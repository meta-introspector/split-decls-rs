macro_rules! deps {
    () => {
        WithCachedTypeInfo!();
    };
}

macro_rules! impl_420 {
    () => {
        deps!();
        impl < T : Ord > PartialOrd for WithCachedTypeInfo < T > { fn partial_cmp (& self , other : & WithCachedTypeInfo < T >) -> Option < Ordering > { Some (self . internee . cmp (& other . internee)) } }
    };
}

impl_420!();