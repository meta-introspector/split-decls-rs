macro_rules! deps {
    () => {
        Config!();
        Generation!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl < C : cfg :: Config > Ord for Generation < C > { fn cmp (& self , other : & Self) -> std :: cmp :: Ordering { self . value . cmp (& other . value) } }
    };
}

impl_94!();