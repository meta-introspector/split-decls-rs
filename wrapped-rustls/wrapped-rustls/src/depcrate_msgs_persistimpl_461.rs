// Generated macro for impl_461 (impl)
macro_rules! Depcrate_msgs_persistimpl_461 {
() => {
// Module: crate::msgs::persist
// Provides: {"impl_461"}
// Dependencies: {}
impl ClientSessionCommon { fn new (ticket : Arc < PayloadU16 > , time_now : UnixTime , lifetime : Duration , peer_identity : Identity < 'static > , server_cert_verifier : & Arc < dyn ServerVerifier > , client_creds : & Arc < dyn ClientCredentialResolver > ,) -> Self { Self { ticket , epoch : time_now . as_secs () , lifetime : cmp :: min (lifetime , MAX_TICKET_LIFETIME) , peer_identity : Arc :: new (peer_identity) , server_cert_verifier : Arc :: downgrade (server_cert_verifier) , client_creds : Arc :: downgrade (client_creds) , } } pub (crate) fn compatible_config (& self , server_cert_verifier : & Arc < dyn ServerVerifier > , client_creds : & Arc < dyn ClientCredentialResolver > ,) -> bool { let same_verifier = Weak :: ptr_eq (& Arc :: downgrade (server_cert_verifier) , & self . server_cert_verifier ,) ; let same_creds = Weak :: ptr_eq (& Arc :: downgrade (client_creds) , & self . client_creds) ; match (same_verifier , same_creds) { (true , true) => true , (false , _) => { crate :: log :: trace ! ("resumption not allowed between different ServerVerifiers") ; false } (_ , _) => { crate :: log :: trace ! ("resumption not allowed between different ClientCredentialResolver values") ; false } } } pub (crate) fn peer_identity (& self) -> & Identity < 'static > { & self . peer_identity } pub (crate) fn ticket (& self) -> & [u8] { self . ticket . 0 . as_ref () } }
};
}
