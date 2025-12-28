macro_rules! assert_not_impl_any {
    () => {
        # [doc = " Asserts that the type does **not** implement _any_ of the given traits."] # [doc = ""] # [doc = " This macro has been deprecated in favor of"] # [doc = " [`assert_impl_not_any!`](macro.assert_impl_not_any.html)."] # [deprecated (since = "1.2.0" , note = "Please use the 'assert_impl_not_any' macro instead")] # [macro_export (local_inner_macros)] macro_rules ! assert_not_impl_any { ($ ($ t : tt) *) => { assert_impl_not_any ! ($ ($ t) *) ; } ; }
    };
}

assert_not_impl_any!();