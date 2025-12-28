macro_rules! ptr_to_repr {
    () => {
        fn ptr_to_repr (original : * mut u8) -> NonNull < u8 > { let modified = (original as usize | 1) . rotate_right (1) ; let diff = modified . wrapping_sub (original as usize) ; let modified = original . wrapping_add (diff) ; unsafe { NonNull :: new_unchecked (modified) } }
    };
}

ptr_to_repr!()