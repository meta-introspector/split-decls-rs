// Generated macro for Tls12ServerSessionValue (struct)
macro_rules! Depcrate_msgs_persistTls12ServerSessionValue {
() => {
// Module: crate::msgs::persist
// Provides: {"Tls12ServerSessionValue"}
// Dependencies: {}
# [derive (Debug)] pub struct Tls12ServerSessionValue { # [doc (hidden)] pub common : CommonServerSessionValue , pub (crate) master_secret : Zeroizing < [u8 ; 48] > , pub (crate) extended_ms : bool , }
};
}
