// Generated macro for test_contiguous (function)
macro_rules! Depcrate_algorithms_myerstest_contiguous {
() => {
// Module: crate::algorithms::myers
// Provides: {"test_contiguous"}
// Dependencies: {}
# [test] fn test_contiguous () { let a : & [usize] = & [0 , 1 , 2 , 3 , 4 , 4 , 4 , 5] ; let b : & [usize] = & [0 , 1 , 2 , 8 , 9 , 4 , 4 , 7] ; let mut d = crate :: algorithms :: Replace :: new (crate :: algorithms :: Capture :: new ()) ; diff (& mut d , a , 0 .. a . len () , b , 0 .. b . len ()) . unwrap () ; insta :: assert_debug_snapshot ! (d . into_inner () . ops ()) ; }
};
}
