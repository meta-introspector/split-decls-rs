// Generated macro for test_replace (function)
macro_rules! Depcrate_algorithms_replacetest_replace {
() => {
// Module: crate::algorithms::replace
// Provides: {"test_replace"}
// Dependencies: {}
# [test] fn test_replace () { use crate :: algorithms :: { diff_slices , Algorithm } ; let a : & [usize] = & [0 , 1 , 2 , 3 , 4] ; let b : & [usize] = & [0 , 1 , 2 , 7 , 8 , 9] ; let mut d = Replace :: new (crate :: algorithms :: Capture :: new ()) ; diff_slices (Algorithm :: Myers , & mut d , a , b) . unwrap () ; insta :: assert_debug_snapshot ! (d . into_inner () . ops () , @ r###"
    [
        Equal {
            old_index: 0,
            new_index: 0,
            len: 3,
        },
        Replace {
            old_index: 3,
            old_len: 2,
            new_index: 3,
            new_len: 3,
        },
    ]
    "###) ; }
};
}
