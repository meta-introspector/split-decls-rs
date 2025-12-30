// Generated macro for Tls13ServerSessionValue (struct)
macro_rules! Depcrate_msgs_persistTls13ServerSessionValue {
() => {
// Module: crate::msgs::persist
// Provides: {"Tls13ServerSessionValue"}
// Dependencies: {}
# [derive (Debug)] pub struct Tls13ServerSessionValue { # [doc (hidden)] pub common : CommonServerSessionValue , pub (crate) secret : Zeroizing < PayloadU8 > , pub (crate) age_obfuscation_offset : u32 , freshness : Option < bool > , }
};
}
