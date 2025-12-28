macro_rules! deps {
    () => {
        RefCount!();
        Config!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl < C : cfg :: Config > Ord for RefCount < C > { fn cmp (& self , other : & Self) -> std :: cmp :: Ordering { self . value . cmp (& other . value) } }
    };
}

impl_109!()