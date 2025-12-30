// Generated macro for CommonServerSessionValue (struct)
macro_rules! Depcrate_msgs_persistCommonServerSessionValue {
() => {
// Module: crate::msgs::persist
// Provides: {"CommonServerSessionValue"}
// Dependencies: {}
# [derive (Debug)] pub struct CommonServerSessionValue { pub (crate) sni : Option < DnsName < 'static > > , pub (crate) cipher_suite : CipherSuite , pub (crate) peer_identity : Option < Identity < 'static > > , pub (crate) alpn : Option < ProtocolName > , pub (crate) application_data : PayloadU16 , # [doc (hidden)] pub creation_time_sec : u64 , }
};
}
