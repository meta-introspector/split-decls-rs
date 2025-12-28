macro_rules! deps {
    () => {
        RefCount!();
        Config!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl < C : cfg :: Config > PartialOrd for RefCount < C > { fn partial_cmp (& self , other : & Self) -> Option < std :: cmp :: Ordering > { Some (self . cmp (other)) } }
    };
}

impl_108!()