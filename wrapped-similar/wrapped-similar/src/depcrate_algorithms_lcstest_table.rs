// Generated macro for test_table (function)
macro_rules! Depcrate_algorithms_lcstest_table {
() => {
// Module: crate::algorithms::lcs
// Provides: {"test_table"}
// Dependencies: {}
# [test] fn test_table () { let table = make_table (& vec ! [2 , 3] , 0 .. 2 , & vec ! [0 , 1 , 2] , 0 .. 3 , None) . unwrap () ; let expected = { let mut m = BTreeMap :: new () ; m . insert ((1 , 0) , 1) ; m . insert ((0 , 0) , 1) ; m . insert ((2 , 0) , 1) ; m } ; assert_eq ! (table , expected) ; }
};
}
