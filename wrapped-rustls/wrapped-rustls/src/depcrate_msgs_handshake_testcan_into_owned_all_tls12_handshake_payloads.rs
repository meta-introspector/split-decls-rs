// Generated macro for can_into_owned_all_tls12_handshake_payloads (function)
macro_rules! Depcrate_msgs_handshake_testcan_into_owned_all_tls12_handshake_payloads {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"can_into_owned_all_tls12_handshake_payloads"}
// Dependencies: {}
# [test] fn can_into_owned_all_tls12_handshake_payloads () { for hm in all_tls12_handshake_payloads () . drain (..) { let enc = hm . get_encoding () ; let debug = format ! ("{hm:?}") ; let other = hm . into_owned () ; assert_eq ! (enc , other . get_encoding ()) ; assert_eq ! (debug , format ! ("{other:?}")) ; } }
};
}
