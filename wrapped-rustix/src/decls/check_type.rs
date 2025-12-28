macro_rules! check_type {
    () => {
        # [doc = " Check that the size and alignment of a type match the `sys` bindings."] macro_rules ! check_type { ($ struct : ident) => { assert_eq_size ! ($ struct , c ::$ struct) ; assert_eq_align ! ($ struct , c ::$ struct) ; } ; }
    };
}

check_type!()