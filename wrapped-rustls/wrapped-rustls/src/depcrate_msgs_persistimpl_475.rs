// Generated macro for impl_475 (impl)
macro_rules! Depcrate_msgs_persistimpl_475 {
() => {
// Module: crate::msgs::persist
// Provides: {"impl_475"}
// Dependencies: {}
impl CommonServerSessionValue { pub (crate) fn new (sni : Option < & DnsName < '_ > > , cipher_suite : CipherSuite , peer_identity : Option < Identity < 'static > > , alpn : Option < ProtocolName > , application_data : Vec < u8 > , creation_time : UnixTime ,) -> Self { Self { sni : sni . map (| s | s . to_owned ()) , cipher_suite , peer_identity , alpn , application_data : PayloadU16 :: new (application_data) , creation_time_sec : creation_time . as_secs () , } } pub (crate) fn can_resume (& self , suite : CipherSuite , sni : & Option < DnsName < '_ > >) -> bool { self . cipher_suite == suite && & self . sni == sni } }
};
}
