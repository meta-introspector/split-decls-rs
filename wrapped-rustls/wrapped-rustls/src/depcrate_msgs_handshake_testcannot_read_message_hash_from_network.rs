// Generated macro for cannot_read_message_hash_from_network (function)
macro_rules! Depcrate_msgs_handshake_testcannot_read_message_hash_from_network {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"cannot_read_message_hash_from_network"}
// Dependencies: {}
# [test] fn cannot_read_message_hash_from_network () { let mh = HandshakeMessagePayload (HandshakePayload :: MessageHash (Payload :: new (vec ! [1 , 2 , 3]))) ; println ! ("mh {mh:?}") ; let enc = mh . get_encoding () ; assert ! (HandshakeMessagePayload :: read_bytes (& enc) . is_err ()) ; }
};
}
