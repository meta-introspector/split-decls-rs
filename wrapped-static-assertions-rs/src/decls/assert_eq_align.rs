macro_rules! assert_eq_align {
    () => {
        # [doc = " Asserts that the types' alignments are equal."] # [doc = ""] # [doc = " This macro has been deprecated in favor of"] # [doc = " [`assert_align_eq!`](macro.assert_align_eq.html)."] # [deprecated (since = "1.2.0" , note = "Please use the 'assert_align_eq' macro instead")] # [macro_export (local_inner_macros)] macro_rules ! assert_eq_align { ($ ($ t : tt) *) => { assert_align_eq ! ($ ($ t) *) ; } ; }
    };
}

assert_eq_align!()