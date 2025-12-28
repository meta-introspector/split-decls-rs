macro_rules! deps {
    () => {
        Key!();
    };
}

macro_rules! impl_133 {
    () => {
        deps!();
        impl PartialOrd for Key { fn partial_cmp (& self , other : & Self) -> Option < std :: cmp :: Ordering > { Some (self . cmp (other)) } }
    };
}

impl_133!();