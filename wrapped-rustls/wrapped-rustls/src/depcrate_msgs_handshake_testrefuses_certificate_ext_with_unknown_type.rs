// Generated macro for refuses_certificate_ext_with_unknown_type (function)
macro_rules! Depcrate_msgs_handshake_testrefuses_certificate_ext_with_unknown_type {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"refuses_certificate_ext_with_unknown_type"}
// Dependencies: {}
# [test] fn refuses_certificate_ext_with_unknown_type () { let bytes = [0x00u8 , 0x08 , 0x00 , 0x05 , 0x00 , 0x03 , 0x99 , 0x00 , 0x00 , 0x00] ; assert_eq ! (CertificateExtensions :: read_bytes (& bytes) . unwrap_err () , InvalidMessage :: InvalidCertificateStatusType) ; }
};
}
