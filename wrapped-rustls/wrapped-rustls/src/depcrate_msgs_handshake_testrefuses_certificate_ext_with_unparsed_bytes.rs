// Generated macro for refuses_certificate_ext_with_unparsed_bytes (function)
macro_rules! Depcrate_msgs_handshake_testrefuses_certificate_ext_with_unparsed_bytes {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"refuses_certificate_ext_with_unparsed_bytes"}
// Dependencies: {}
# [test] fn refuses_certificate_ext_with_unparsed_bytes () { let bytes = [0x00u8 , 0x0a , 0x00 , 0x05 , 0x00 , 0x06 , 0x01 , 0x00 , 0x00 , 0x01 , 0xcc , 0x01 ,] ; assert_eq ! (CertificateExtensions :: read_bytes (& bytes) . unwrap_err () , InvalidMessage :: TrailingData ("CertificateExtensions")) ; }
};
}
