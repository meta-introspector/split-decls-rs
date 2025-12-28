macro_rules! deps {
    () => {
        Segment!();
    };
}

macro_rules! impl_205 {
    () => {
        deps!();
        impl Ord for Segment { fn cmp (& self , other : & Self) -> core :: cmp :: Ordering { self . score . cmp (& other . score) } }
    };
}

impl_205!()