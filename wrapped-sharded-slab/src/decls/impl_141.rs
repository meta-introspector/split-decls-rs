macro_rules! deps {
    () => {
        Config!();
        Addr!();
    };
}

macro_rules! impl_141 {
    () => {
        deps!();
        impl < C : cfg :: Config > Ord for Addr < C > { fn cmp (& self , other : & Self) -> std :: cmp :: Ordering { self . addr . cmp (& other . addr) } }
    };
}

impl_141!()