macro_rules! repr_to_ptr {
    () => {
        fn repr_to_ptr (modified : NonNull < u8 >) -> * const u8 { let modified = modified . as_ptr () ; let original = (modified as usize) << 1 ; let diff = original . wrapping_sub (modified as usize) ; modified . wrapping_add (diff) }
    };
}

repr_to_ptr!();