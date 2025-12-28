macro_rules! deps {
    () => {
        RedactedValueInner!();
    };
}

macro_rules! impl_238 {
    () => {
        deps!();
        impl Ord for RedactedValueInner { fn cmp (& self , other : & Self) -> std :: cmp :: Ordering { self . as_cmp () . cmp (& other . as_cmp ()) } }
    };
}

impl_238!();