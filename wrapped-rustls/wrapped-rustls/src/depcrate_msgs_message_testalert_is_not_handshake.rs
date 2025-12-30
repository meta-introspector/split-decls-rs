// Generated macro for alert_is_not_handshake (function)
macro_rules! Depcrate_msgs_message_testalert_is_not_handshake {
() => {
// Module: crate::msgs::message_test
// Provides: {"alert_is_not_handshake"}
// Dependencies: {}
# [test] fn alert_is_not_handshake () { let m = Message :: build_alert (AlertLevel :: Fatal , AlertDescription :: DecodeError) ; assert_ne ! (m . handshake_type () , Some (HandshakeType :: ClientHello)) ; }
};
}
