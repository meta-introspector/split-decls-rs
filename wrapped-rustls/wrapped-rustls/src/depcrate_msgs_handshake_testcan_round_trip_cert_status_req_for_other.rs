// Generated macro for can_round_trip_cert_status_req_for_other (function)
macro_rules! Depcrate_msgs_handshake_testcan_round_trip_cert_status_req_for_other {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"can_round_trip_cert_status_req_for_other"}
// Dependencies: {}
# [test] fn can_round_trip_cert_status_req_for_other () { let bytes = [0 , 5 , 2 , 1 , 2 , 3 , 4 ,] ; let csr = CertificateStatusRequest :: read (& mut Reader :: init (& bytes)) . unwrap () ; println ! ("{csr:?}") ; assert_eq ! (csr . get_encoding () , bytes . to_vec ()) ; }
};
}
