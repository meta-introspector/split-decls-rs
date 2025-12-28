macro_rules! deps {
    () => {
        Unicode!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < T : AsRef < str > > Ord for Unicode < T > { # [inline] fn cmp (& self , other : & Self) -> Ordering { let self_chars = self . 0 . as_ref () . chars () . flat_map (lookup) ; let other_chars = other . 0 . as_ref () . chars () . flat_map (lookup) ; self_chars . cmp (other_chars) } }
    };
}

impl_26!()