macro_rules! deps {
    () => {
        HSTRING!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl PartialOrd for HSTRING { fn partial_cmp (& self , other : & Self) -> Option < core :: cmp :: Ordering > { Some (self . cmp (other)) } }
    };
}

impl_38!();