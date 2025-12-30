// Generated macro for refuses_new_session_ticket_ext_with_duplicate_extension (function)
macro_rules! Depcrate_msgs_handshake_testrefuses_new_session_ticket_ext_with_duplicate_extension {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"refuses_new_session_ticket_ext_with_duplicate_extension"}
// Dependencies: {}
# [test] fn refuses_new_session_ticket_ext_with_duplicate_extension () { let bytes = [0x00u8 , 0x08 , 0x00 , 0x99 , 0x00 , 0x00 , 0x00 , 0x99 , 0x00 , 0x00] ; assert_eq ! (NewSessionTicketExtensions :: read_bytes (& bytes) . unwrap_err () , InvalidMessage :: DuplicateExtension (0x0099)) ; }
};
}
