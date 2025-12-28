macro_rules! assert_not_impl_all {
    () => {
        # [doc = " Asserts that the type does **not** implement _all_ of the given traits."] # [doc = ""] # [doc = " This macro has been deprecated in favor of"] # [doc = " [`assert_impl_not_all!`](macro.assert_impl_not_all.html)."] # [deprecated (since = "1.2.0" , note = "Please use the 'assert_impl_not_all' macro instead")] # [macro_export (local_inner_macros)] macro_rules ! assert_not_impl_all { ($ ($ t : tt) *) => { assert_impl_not_all ! ($ ($ t) *) ; } ; }
    };
}

assert_not_impl_all!()