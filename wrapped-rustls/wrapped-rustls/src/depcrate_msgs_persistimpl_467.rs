// Generated macro for impl_467 (impl)
macro_rules! Depcrate_msgs_persistimpl_467 {
() => {
// Module: crate::msgs::persist
// Provides: {"impl_467"}
// Dependencies: {}
impl Tls12ServerSessionValue { pub (crate) fn new (common : CommonServerSessionValue , master_secret : & [u8 ; 48] , extended_ms : bool ,) -> Self { Self { common , master_secret : Zeroizing :: new (* master_secret) , extended_ms , } } }
};
}
