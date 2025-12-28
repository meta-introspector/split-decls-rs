macro_rules! deps {
    () => {
        Addr!();
        Config!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        impl < C : cfg :: Config > PartialOrd for Addr < C > { fn partial_cmp (& self , other : & Self) -> Option < std :: cmp :: Ordering > { Some (self . cmp (other)) } }
    };
}

impl_140!();