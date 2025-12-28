macro_rules! deps {
    () => {
        Generation!();
        Config!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl < C : cfg :: Config > PartialOrd for Generation < C > { fn partial_cmp (& self , other : & Self) -> Option < std :: cmp :: Ordering > { Some (self . cmp (other)) } }
    };
}

impl_93!()