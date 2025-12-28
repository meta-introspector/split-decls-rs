macro_rules! deps {
    () => {
        Spanned!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl < T : Ord > Ord for Spanned < T > { fn cmp (& self , other : & Self) -> Ordering { self . value . cmp (& other . value) } }
    };
}

impl_18!();