macro_rules! deps {
    () => {
        Spanned!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < T : PartialOrd > PartialOrd for Spanned < T > { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { self . value . partial_cmp (& other . value) } }
    };
}

impl_17!();