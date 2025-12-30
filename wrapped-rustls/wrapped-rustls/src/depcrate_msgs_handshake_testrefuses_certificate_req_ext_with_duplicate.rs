// Generated macro for refuses_certificate_req_ext_with_duplicate (function)
macro_rules! Depcrate_msgs_handshake_testrefuses_certificate_req_ext_with_duplicate {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"refuses_certificate_req_ext_with_duplicate"}
// Dependencies: {}
# [test] fn refuses_certificate_req_ext_with_duplicate () { let bytes = [0x00u8 , 0x08 , 0x00 , 0x99 , 0x00 , 0x00 , 0x00 , 0x99 , 0x00 , 0x00] ; assert_eq ! (CertificateRequestExtensions :: read_bytes (& bytes) . unwrap_err () , InvalidMessage :: DuplicateExtension (0x0099)) ; }
};
}
