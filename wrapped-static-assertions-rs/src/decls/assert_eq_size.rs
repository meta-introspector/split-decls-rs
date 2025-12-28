macro_rules! assert_eq_size {
    () => {
        # [doc = " Asserts that types are equal in alignment."] # [doc = ""] # [doc = " This macro has been deprecated in favor of"] # [doc = " [`assert_size_eq!`](macro.assert_size_eq.html)."] # [deprecated (since = "1.2.0" , note = "Please use the 'assert_size_eq' macro instead")] # [macro_export (local_inner_macros)] macro_rules ! assert_eq_size { ($ ($ t : tt) *) => { assert_size_eq ! ($ ($ t) *) ; } ; }
    };
}

assert_eq_size!();