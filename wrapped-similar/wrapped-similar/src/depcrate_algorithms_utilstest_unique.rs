// Generated macro for test_unique (function)
macro_rules! Depcrate_algorithms_utilstest_unique {
() => {
// Module: crate::algorithms::utils
// Provides: {"test_unique"}
// Dependencies: {}
# [test] fn test_unique () { let u = unique (& vec ! ['a' , 'b' , 'c' , 'd' , 'd' , 'b'] , 0 .. 6) . into_iter () . map (| x | (* x . value () , x . original_index ())) . collect :: < Vec < _ > > () ; assert_eq ! (u , vec ! [('a' , 0) , ('c' , 2)]) ; }
};
}
