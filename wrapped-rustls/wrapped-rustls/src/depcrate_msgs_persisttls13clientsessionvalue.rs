// Generated macro for Tls13ClientSessionValue (struct)
macro_rules! Depcrate_msgs_persistTls13ClientSessionValue {
() => {
// Module: crate::msgs::persist
// Provides: {"Tls13ClientSessionValue"}
// Dependencies: {}
# [derive (Debug)] pub struct Tls13ClientSessionValue { suite : & 'static Tls13CipherSuite , secret : Zeroizing < PayloadU8 > , age_add : u32 , max_early_data_size : u32 , pub (crate) common : ClientSessionCommon , quic_params : PayloadU16 , }
};
}
