// Generated macro for test_same (function)
macro_rules! Depcrate_algorithms_lcstest_same {
() => {
// Module: crate::algorithms::lcs
// Provides: {"test_same"}
// Dependencies: {}
# [test] fn test_same () { let a : & [usize] = & [0 , 1 , 2 , 3 , 4 , 4 , 4 , 5] ; let b : & [usize] = & [0 , 1 , 2 , 3 , 4 , 4 , 4 , 5] ; let mut d = crate :: algorithms :: Capture :: new () ; diff (& mut d , a , 0 .. a . len () , b , 0 .. b . len ()) . unwrap () ; insta :: assert_debug_snapshot ! (d . ops ()) ; }
};
}
