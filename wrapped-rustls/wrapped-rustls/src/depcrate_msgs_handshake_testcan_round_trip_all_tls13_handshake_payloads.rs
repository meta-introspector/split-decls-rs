// Generated macro for can_round_trip_all_tls13_handshake_payloads (function)
macro_rules! Depcrate_msgs_handshake_testcan_round_trip_all_tls13_handshake_payloads {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"can_round_trip_all_tls13_handshake_payloads"}
// Dependencies: {}
# [test] fn can_round_trip_all_tls13_handshake_payloads () { for hm in all_tls13_handshake_payloads () . iter () { println ! ("{:?}" , hm . 0 . handshake_type ()) ; let bytes = hm . get_encoding () ; let mut rd = Reader :: init (& bytes) ; let other = HandshakeMessagePayload :: read_version (& mut rd , ProtocolVersion :: TLSv1_3) . unwrap () ; assert ! (! rd . any_left ()) ; assert_eq ! (hm . get_encoding () , other . get_encoding ()) ; println ! ("{hm:?}") ; println ! ("{other:?}") ; } }
};
}
