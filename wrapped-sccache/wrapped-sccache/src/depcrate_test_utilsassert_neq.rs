// Generated macro for assert_neq (macro)
macro_rules! Depcrate_test_utilsassert_neq {
() => {
// Module: crate::test::utils
// Provides: {"assert_neq"}
// Dependencies: {}
# [doc = " Assert that `left != right`."] macro_rules ! assert_neq { ($ left : expr , $ right : expr) => { { match (& ($ left) , & ($ right)) { (left_val , right_val) => { if ! (* left_val != * right_val) { panic ! ("assertion failed: `(left != right)` \
                         (left: `{:?}`, right: `{:?}`)" , left_val , right_val) } } } } } ; }
};
}
