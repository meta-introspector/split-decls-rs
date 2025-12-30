// Generated macro for sample_certificate_status (function)
macro_rules! Depcrate_msgs_handshake_testsample_certificate_status {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"sample_certificate_status"}
// Dependencies: {}
fn sample_certificate_status () -> CertificateStatus < 'static > { CertificateStatus { ocsp_response : PayloadU24 :: from (Payload :: new (vec ! [1 , 2 , 3])) , } }
};
}
