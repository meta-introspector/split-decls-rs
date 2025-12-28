macro_rules! deps {
    () => {
        Segment!();
    };
}

macro_rules! impl_204 {
    () => {
        deps!();
        impl PartialOrd for Segment { fn partial_cmp (& self , other : & Self) -> Option < core :: cmp :: Ordering > { Some (self . cmp (other)) } }
    };
}

impl_204!();