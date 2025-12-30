// Generated macro for impl_108 (impl)
macro_rules! Depcrate_rc_test_objectimpl_108 {
() => {
// Module: crate::rc_test_object
// Provides: {"impl_108"}
// Dependencies: {}
impl RcTestObject { # [doc (hidden)] # [allow (dead_code)] pub (crate) fn new () -> Retained < Self > { unsafe { Retained :: from_raw (msg_send ! [Self :: class () , new]) } . unwrap () } }
};
}
