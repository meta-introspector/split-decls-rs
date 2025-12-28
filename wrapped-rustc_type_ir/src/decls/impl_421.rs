macro_rules! deps {
    () => {
        WithCachedTypeInfo!();
    };
}

macro_rules! impl_421 {
    () => {
        deps!();
        impl < T : Ord > Ord for WithCachedTypeInfo < T > { fn cmp (& self , other : & WithCachedTypeInfo < T >) -> Ordering { self . internee . cmp (& other . internee) } }
    };
}

impl_421!();