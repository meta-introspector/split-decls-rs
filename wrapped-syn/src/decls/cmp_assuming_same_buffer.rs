macro_rules! deps {
    () => {
        Cursor!();
    };
}

macro_rules! cmp_assuming_same_buffer {
    () => {
        deps!();
        pub (crate) fn cmp_assuming_same_buffer (a : Cursor , b : Cursor) -> Ordering { a . ptr . cmp (& b . ptr) }
    };
}

cmp_assuming_same_buffer!();