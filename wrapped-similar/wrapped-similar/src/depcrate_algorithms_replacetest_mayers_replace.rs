// Generated macro for test_mayers_replace (function)
macro_rules! Depcrate_algorithms_replacetest_mayers_replace {
() => {
// Module: crate::algorithms::replace
// Provides: {"test_mayers_replace"}
// Dependencies: {}
# [test] fn test_mayers_replace () { use crate :: algorithms :: { diff_slices , Algorithm } ; let a : & [& str] = & [">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>\n" , "a\n" , "b\n" , "c\n" , "================================\n" , "d\n" , "e\n" , "f\n" , "<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<\n" ,] ; let b : & [& str] = & [">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>\n" , "x\n" , "b\n" , "c\n" , "================================\n" , "y\n" , "e\n" , "f\n" , "<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<\n" ,] ; let mut d = Replace :: new (crate :: algorithms :: Capture :: new ()) ; diff_slices (Algorithm :: Myers , & mut d , a , b) . unwrap () ; insta :: assert_debug_snapshot ! (& d . into_inner () . ops () , @ r###"
    [
        Equal {
            old_index: 0,
            new_index: 0,
            len: 1,
        },
        Replace {
            old_index: 1,
            old_len: 1,
            new_index: 1,
            new_len: 1,
        },
        Equal {
            old_index: 2,
            new_index: 2,
            len: 3,
        },
        Replace {
            old_index: 5,
            old_len: 1,
            new_index: 5,
            new_len: 1,
        },
        Equal {
            old_index: 6,
            new_index: 6,
            len: 3,
        },
    ]
    "###) ; }
};
}
