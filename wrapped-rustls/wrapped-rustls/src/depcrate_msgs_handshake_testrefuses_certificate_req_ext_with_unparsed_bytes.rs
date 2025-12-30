// Generated macro for refuses_certificate_req_ext_with_unparsed_bytes (function)
macro_rules! Depcrate_msgs_handshake_testrefuses_certificate_req_ext_with_unparsed_bytes {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"refuses_certificate_req_ext_with_unparsed_bytes"}
// Dependencies: {}
# [test] fn refuses_certificate_req_ext_with_unparsed_bytes () { let bytes = [0x00u8 , 0x09 , 0x00 , 0x0d , 0x00 , 0x05 , 0x00 , 0x02 , 0x01 , 0x02 , 0xff ,] ; assert_eq ! (CertificateRequestExtensions :: read_bytes (& bytes) . unwrap_err () , InvalidMessage :: TrailingData ("CertificateRequestExtensions")) ; }
};
}
