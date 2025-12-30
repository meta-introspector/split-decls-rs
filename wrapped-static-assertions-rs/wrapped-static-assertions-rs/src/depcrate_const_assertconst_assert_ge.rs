// Generated macro for const_assert_ge (macro)
macro_rules! Depcrate_const_assertconst_assert_ge {
() => {
// Module: crate::const_assert
// Provides: {"const_assert_ge"}
// Dependencies: {}
# [doc = " Asserts that constants are less than or equal to each other."] # [macro_export (local_inner_macros)] macro_rules ! const_assert_ge { ($ x : expr , $ ($ y : expr) ,+ $ (,) ?) => { const_assert_ge ! (@ build $ x , $ ($ y) ,+) ; } ; (@ build $ x : expr) => { } ; (@ build $ x : expr , $ ($ y : expr) ,+) => { const_assert ! ($ x >= _head ! ($ ($ y) ,+)) ; const_assert_ge ! (@ build $ ($ y) ,+) ; } ; }
};
}
