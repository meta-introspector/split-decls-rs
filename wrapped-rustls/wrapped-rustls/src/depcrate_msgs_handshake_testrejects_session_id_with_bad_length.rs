// Generated macro for rejects_session_id_with_bad_length (function)
macro_rules! Depcrate_msgs_handshake_testrejects_session_id_with_bad_length {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"rejects_session_id_with_bad_length"}
// Dependencies: {}
# [test] fn rejects_session_id_with_bad_length () { let bytes = [33 ; 33] ; let mut rd = Reader :: init (& bytes) ; assert ! (SessionId :: read (& mut rd) . is_err ()) ; }
};
}
