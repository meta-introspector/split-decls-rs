// Generated macro for const_assert_le (macro)
macro_rules! Depcrate_const_assertconst_assert_le {
() => {
// Module: crate::const_assert
// Provides: {"const_assert_le"}
// Dependencies: {}
# [doc = " Asserts that constants are less than or equal to each other."] # [macro_export (local_inner_macros)] macro_rules ! const_assert_le { ($ x : expr , $ ($ y : expr) ,+ $ (,) ?) => { const_assert_le ! (@ build $ x , $ ($ y) ,+) ; } ; (@ build $ x : expr) => { } ; (@ build $ x : expr , $ ($ y : expr) ,+) => { const_assert ! ($ x <= _head ! ($ ($ y) ,+)) ; const_assert_le ! (@ build $ ($ y) ,+) ; } ; }
};
}
