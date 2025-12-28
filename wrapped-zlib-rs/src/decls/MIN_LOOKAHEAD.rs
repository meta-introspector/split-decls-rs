macro_rules! MIN_LOOKAHEAD {
    () => {
        pub (crate) const MIN_LOOKAHEAD : usize = STD_MAX_MATCH + STD_MIN_MATCH + 1 ;
    };
}

MIN_LOOKAHEAD!();