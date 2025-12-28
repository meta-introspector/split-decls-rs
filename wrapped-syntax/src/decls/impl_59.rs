macro_rules! deps {
    () => {
        TokenText!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl Ord for TokenText < '_ > { fn cmp (& self , other : & Self) -> Ordering { self . as_str () . cmp (other . as_str ()) } }
    };
}

impl_59!();