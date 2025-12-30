// Generated macro for sample_hello_retry_request (function)
macro_rules! Depcrate_msgs_handshake_testsample_hello_retry_request {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"sample_hello_retry_request"}
// Dependencies: {}
fn sample_hello_retry_request () -> HelloRetryRequest { HelloRetryRequest { legacy_version : ProtocolVersion :: TLSv1_2 , session_id : SessionId :: empty () , cipher_suite : CipherSuite :: TLS_PSK_DHE_WITH_AES_128_CCM_8 , extensions : HelloRetryRequestExtensions { key_share : Some (NamedGroup :: X25519) , cookie : Some (PayloadU16 :: new (vec ! [0])) , supported_versions : Some (ProtocolVersion :: TLSv1_2) , encrypted_client_hello : Some (Payload :: new (vec ! [1 , 2 , 3])) , order : None , } , } }
};
}
