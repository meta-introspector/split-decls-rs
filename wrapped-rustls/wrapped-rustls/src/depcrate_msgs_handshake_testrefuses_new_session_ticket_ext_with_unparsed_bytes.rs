// Generated macro for refuses_new_session_ticket_ext_with_unparsed_bytes (function)
macro_rules! Depcrate_msgs_handshake_testrefuses_new_session_ticket_ext_with_unparsed_bytes {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"refuses_new_session_ticket_ext_with_unparsed_bytes"}
// Dependencies: {}
# [test] fn refuses_new_session_ticket_ext_with_unparsed_bytes () { let bytes = [0x00u8 , 0x09 , 0x00 , 0x2a , 0x00 , 0x05 , 0x00 , 0x00 , 0x00 , 0x00 , 0x01 ,] ; assert_eq ! (NewSessionTicketExtensions :: read_bytes (& bytes) . unwrap_err () , InvalidMessage :: TrailingData ("NewSessionTicketExtensions")) ; }
};
}
