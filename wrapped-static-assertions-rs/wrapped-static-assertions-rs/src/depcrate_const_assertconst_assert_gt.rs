// Generated macro for const_assert_gt (macro)
macro_rules! Depcrate_const_assertconst_assert_gt {
() => {
// Module: crate::const_assert
// Provides: {"const_assert_gt"}
// Dependencies: {}
# [doc = " Asserts that constants are greater than each other."] # [macro_export (local_inner_macros)] macro_rules ! const_assert_gt { ($ x : expr , $ ($ y : expr) ,+ $ (,) ?) => { const_assert_gt ! (@ build $ x , $ ($ y) ,+) ; } ; (@ build $ x : expr) => { } ; (@ build $ x : expr , $ ($ y : expr) ,+) => { const_assert ! ($ x > _head ! ($ ($ y) ,+)) ; const_assert_gt ! (@ build $ ($ y) ,+) ; } ; }
};
}
