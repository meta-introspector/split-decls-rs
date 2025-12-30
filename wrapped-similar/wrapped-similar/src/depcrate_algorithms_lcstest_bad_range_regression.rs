// Generated macro for test_bad_range_regression (function)
macro_rules! Depcrate_algorithms_lcstest_bad_range_regression {
() => {
// Module: crate::algorithms::lcs
// Provides: {"test_bad_range_regression"}
// Dependencies: {}
# [test] fn test_bad_range_regression () { use crate :: algorithms :: Capture ; use crate :: DiffOp ; let mut d = Capture :: new () ; diff (& mut d , & [0] , 0 .. 1 , & [0 , 0] , 0 .. 2) . unwrap () ; assert_eq ! (d . into_ops () , vec ! [DiffOp :: Equal { old_index : 0 , new_index : 0 , len : 1 } , DiffOp :: Insert { old_index : 1 , new_index : 1 , new_len : 1 }]) ; }
};
}
