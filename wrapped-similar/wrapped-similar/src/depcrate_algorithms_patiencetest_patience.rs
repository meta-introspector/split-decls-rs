// Generated macro for test_patience (function)
macro_rules! Depcrate_algorithms_patiencetest_patience {
() => {
// Module: crate::algorithms::patience
// Provides: {"test_patience"}
// Dependencies: {}
# [test] fn test_patience () { let a : & [usize] = & [11 , 1 , 2 , 2 , 3 , 4 , 4 , 4 , 5 , 47 , 19] ; let b : & [usize] = & [10 , 1 , 2 , 2 , 8 , 9 , 4 , 4 , 7 , 47 , 18] ; let mut d = Replace :: new (crate :: algorithms :: Capture :: new ()) ; diff (& mut d , a , 0 .. a . len () , b , 0 .. b . len ()) . unwrap () ; insta :: assert_debug_snapshot ! (d . into_inner () . ops ()) ; }
};
}
