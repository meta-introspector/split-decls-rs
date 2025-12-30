// Generated macro for test_patience_out_of_bounds_bug (function)
macro_rules! Depcrate_algorithms_patiencetest_patience_out_of_bounds_bug {
() => {
// Module: crate::algorithms::patience
// Provides: {"test_patience_out_of_bounds_bug"}
// Dependencies: {}
# [test] fn test_patience_out_of_bounds_bug () { let a : & [usize] = & [1 , 2 , 3 , 4] ; let b : & [usize] = & [1 , 2 , 3] ; let mut d = Replace :: new (crate :: algorithms :: Capture :: new ()) ; diff (& mut d , a , 0 .. a . len () , b , 0 .. b . len ()) . unwrap () ; insta :: assert_debug_snapshot ! (d . into_inner () . ops ()) ; }
};
}
