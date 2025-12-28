macro_rules! deps {
    () => {
        Index!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl PartialOrd for Index { # [inline (always)] fn partial_cmp (& self , other : & Self) -> Option < Ordering > { self . 0 . partial_cmp (& other . 0) } }
    };
}

impl_38!();