// Generated macro for test_diff (function)
macro_rules! Depcrate_algorithms_myerstest_diff {
() => {
// Module: crate::algorithms::myers
// Provides: {"test_diff"}
// Dependencies: {}
# [test] fn test_diff () { let a : & [usize] = & [0 , 1 , 2 , 3 , 4] ; let b : & [usize] = & [0 , 1 , 2 , 9 , 4] ; let mut d = crate :: algorithms :: Replace :: new (crate :: algorithms :: Capture :: new ()) ; diff (& mut d , a , 0 .. a . len () , b , 0 .. b . len ()) . unwrap () ; insta :: assert_debug_snapshot ! (d . into_inner () . ops ()) ; }
};
}
