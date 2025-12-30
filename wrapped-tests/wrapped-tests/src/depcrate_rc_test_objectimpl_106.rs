// Generated macro for impl_106 (impl)
macro_rules! Depcrate_rc_test_objectimpl_106 {
() => {
// Module: crate::rc_test_object
// Provides: {"impl_106"}
// Dependencies: {}
impl Drop for RcTestObject { fn drop (& mut self) { TEST_DATA . with (| data | data . borrow_mut () . drop += 1) ; } }
};
}
