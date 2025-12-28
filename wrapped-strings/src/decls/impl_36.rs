macro_rules! deps {
    () => {
        HSTRING!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl Ord for HSTRING { fn cmp (& self , other : & Self) -> core :: cmp :: Ordering { self . deref () . cmp (other) } }
    };
}

impl_36!();