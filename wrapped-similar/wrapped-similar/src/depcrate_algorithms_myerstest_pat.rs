// Generated macro for test_pat (function)
macro_rules! Depcrate_algorithms_myerstest_pat {
() => {
// Module: crate::algorithms::myers
// Provides: {"test_pat"}
// Dependencies: {}
# [test] fn test_pat () { let a : & [usize] = & [0 , 1 , 3 , 4 , 5] ; let b : & [usize] = & [0 , 1 , 4 , 5 , 8 , 9] ; let mut d = crate :: algorithms :: Capture :: new () ; diff (& mut d , a , 0 .. a . len () , b , 0 .. b . len ()) . unwrap () ; insta :: assert_debug_snapshot ! (d . ops ()) ; }
};
}
