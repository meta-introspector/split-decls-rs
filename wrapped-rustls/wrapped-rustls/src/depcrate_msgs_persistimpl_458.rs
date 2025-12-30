// Generated macro for impl_458 (impl)
macro_rules! Depcrate_msgs_persistimpl_458 {
() => {
// Module: crate::msgs::persist
// Provides: {"impl_458"}
// Dependencies: {}
impl Tls12ClientSessionValue { pub (crate) fn new (suite : & 'static Tls12CipherSuite , session_id : SessionId , ticket : Arc < PayloadU16 > , master_secret : & [u8 ; 48] , peer_identity : Identity < 'static > , server_cert_verifier : & Arc < dyn ServerVerifier > , client_creds : & Arc < dyn ClientCredentialResolver > , time_now : UnixTime , lifetime : Duration , extended_ms : bool ,) -> Self { Self { suite , session_id , master_secret : Zeroizing :: new (* master_secret) , extended_ms , common : ClientSessionCommon :: new (ticket , time_now , lifetime , peer_identity , server_cert_verifier , client_creds ,) , } } pub (crate) fn master_secret (& self) -> & [u8 ; 48] { & self . master_secret } pub (crate) fn ticket (& mut self) -> Arc < PayloadU16 > { self . common . ticket . clone () } pub (crate) fn extended_ms (& self) -> bool { self . extended_ms } pub (crate) fn suite (& self) -> & 'static Tls12CipherSuite { self . suite } # [doc = " Test only: rewind epoch by `delta` seconds."] # [doc (hidden)] pub fn rewind_epoch (& mut self , delta : u32) { self . common . epoch -= delta as u64 ; } }
};
}
