// Generated macro for ClientSessionCommon (struct)
macro_rules! Depcrate_msgs_persistClientSessionCommon {
() => {
// Module: crate::msgs::persist
// Provides: {"ClientSessionCommon"}
// Dependencies: {}
# [derive (Debug , Clone)] pub struct ClientSessionCommon { ticket : Arc < PayloadU16 > , epoch : u64 , lifetime : Duration , peer_identity : Arc < Identity < 'static > > , server_cert_verifier : Weak < dyn ServerVerifier > , client_creds : Weak < dyn ClientCredentialResolver > , }
};
}
