macro_rules! assert_eq_size_val {
    () => {
        # [doc = " Asserts that values pointed to are equal in size."] # [doc = ""] # [doc = " This macro has been deprecated in favor of"] # [doc = " [`assert_size_eq_val!`](macro.assert_size_eq_val.html)."] # [deprecated (since = "1.2.0" , note = "Please use the 'assert_size_eq_val' macro instead")] # [macro_export (local_inner_macros)] macro_rules ! assert_eq_size_val { ($ ($ t : tt) *) => { assert_size_eq_val ! ($ ($ t) *) ; } ; }
    };
}

assert_eq_size_val!();