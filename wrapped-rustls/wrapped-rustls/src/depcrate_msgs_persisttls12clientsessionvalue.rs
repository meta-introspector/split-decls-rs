// Generated macro for Tls12ClientSessionValue (struct)
macro_rules! Depcrate_msgs_persistTls12ClientSessionValue {
() => {
// Module: crate::msgs::persist
// Provides: {"Tls12ClientSessionValue"}
// Dependencies: {}
# [derive (Debug , Clone)] pub struct Tls12ClientSessionValue { suite : & 'static Tls12CipherSuite , pub (crate) session_id : SessionId , master_secret : Zeroizing < [u8 ; 48] > , extended_ms : bool , # [doc (hidden)] pub (crate) common : ClientSessionCommon , }
};
}
