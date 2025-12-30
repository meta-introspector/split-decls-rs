// Generated macro for can_round_trip_cert_status_req_for_ocsp (function)
macro_rules! Depcrate_msgs_handshake_testcan_round_trip_cert_status_req_for_ocsp {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"can_round_trip_cert_status_req_for_ocsp"}
// Dependencies: {}
# [test] fn can_round_trip_cert_status_req_for_ocsp () { let ext = CertificateStatusRequest :: build_ocsp () ; println ! ("{ext:?}") ; let bytes = [0 , 11 , 1 , 0 , 5 , 0 , 3 , 0 , 1 , 1 , 0 , 1 , 2 ,] ; let csr = CertificateStatusRequest :: read (& mut Reader :: init (& bytes)) . unwrap () ; println ! ("{csr:?}") ; assert_eq ! (csr . get_encoding () , bytes . to_vec ()) ; }
};
}
