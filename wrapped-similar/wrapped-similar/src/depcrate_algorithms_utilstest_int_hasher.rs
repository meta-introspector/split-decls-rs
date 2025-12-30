// Generated macro for test_int_hasher (function)
macro_rules! Depcrate_algorithms_utilstest_int_hasher {
() => {
// Module: crate::algorithms::utils
// Provides: {"test_int_hasher"}
// Dependencies: {}
# [test] fn test_int_hasher () { let ih = IdentifyDistinct :: < u8 > :: new (& ["" , "foo" , "bar" , "baz"] [..] , 1 .. 4 , & ["" , "foo" , "blah" , "baz"] [..] , 1 .. 4 ,) ; assert_eq ! (ih . old_lookup () [1] , 0) ; assert_eq ! (ih . old_lookup () [2] , 1) ; assert_eq ! (ih . old_lookup () [3] , 2) ; assert_eq ! (ih . new_lookup () [1] , 0) ; assert_eq ! (ih . new_lookup () [2] , 3) ; assert_eq ! (ih . new_lookup () [3] , 2) ; assert_eq ! (ih . old_range () , 1 .. 4) ; assert_eq ! (ih . new_range () , 1 .. 4) ; }
};
}
