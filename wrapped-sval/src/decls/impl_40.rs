macro_rules! deps {
    () => {
        Index!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl Ord for Index { # [inline (always)] fn cmp (& self , other : & Self) -> Ordering { self . 0 . cmp (& other . 0) } }
    };
}

impl_40!();