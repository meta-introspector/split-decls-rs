macro_rules! deps {
    () => {
        RedactedValueInner!();
    };
}

macro_rules! impl_237 {
    () => {
        deps!();
        impl PartialOrd for RedactedValueInner { fn partial_cmp (& self , other : & Self) -> Option < std :: cmp :: Ordering > { Some (self . cmp (other)) } }
    };
}

impl_237!();