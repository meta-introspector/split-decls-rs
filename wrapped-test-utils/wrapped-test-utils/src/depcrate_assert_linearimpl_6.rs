// Generated macro for impl_6 (impl)
macro_rules! Depcrate_assert_linearimpl_6 {
() => {
// Module: crate::assert_linear
// Provides: {"impl_6"}
// Dependencies: {}
impl Drop for AssertLinear { fn drop (& mut self) { assert ! (! self . rounds . is_empty ()) ; if self . rounds . iter () . all (| it | ! it . linear) { for round in & self . rounds { eprintln ! ("\n{}" , round . plot) ; } panic ! ("Doesn't look linear!") ; } } }
};
}
