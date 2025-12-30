// Generated macro for assert_bom (function)
macro_rules! Depcrate_testassert_bom {
() => {
// Module: crate::test
// Provides: {"assert_bom"}
// Dependencies: {}
fn assert_bom (bytes : & [u8] , expected : Bom) { assert_eq ! (Bom :: from (bytes) , expected) ; }
};
}
