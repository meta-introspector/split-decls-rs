// Generated macro for self_signed_cert_manual_trust (function)
macro_rules! Depcrate_testself_signed_cert_manual_trust {
() => {
// Module: crate::test
// Provides: {"self_signed_cert_manual_trust"}
// Dependencies: {}
# [test] fn self_signed_cert_manual_trust () { let cert = include_bytes ! ("../test/self-signed.badssl.com.cer") ; let mut store = Memory :: new () . unwrap () ; store . add_encoded_certificate (cert) . unwrap () ; let creds = SchannelCred :: builder () . acquire (Direction :: Outbound) . unwrap () ; let stream = TcpStream :: connect ("self-signed.badssl.com:443") . unwrap () ; tls_stream :: Builder :: new () . domain ("self-signed.badssl.com") . cert_store (store . into_store ()) . connect (creds , stream) . unwrap () ; }
};
}
