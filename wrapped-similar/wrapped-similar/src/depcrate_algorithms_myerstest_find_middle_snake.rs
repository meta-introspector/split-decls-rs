// Generated macro for test_find_middle_snake (function)
macro_rules! Depcrate_algorithms_myerstest_find_middle_snake {
() => {
// Module: crate::algorithms::myers
// Provides: {"test_find_middle_snake"}
// Dependencies: {}
# [test] fn test_find_middle_snake () { let a = & b"ABCABBA" [..] ; let b = & b"CBABAC" [..] ; let max_d = max_d (a . len () , b . len ()) ; let mut vf = V :: new (max_d) ; let mut vb = V :: new (max_d) ; let (x_start , y_start) = find_middle_snake (a , 0 .. a . len () , b , 0 .. b . len () , & mut vf , & mut vb , None) . unwrap () ; assert_eq ! (x_start , 4) ; assert_eq ! (y_start , 1) ; }
};
}
