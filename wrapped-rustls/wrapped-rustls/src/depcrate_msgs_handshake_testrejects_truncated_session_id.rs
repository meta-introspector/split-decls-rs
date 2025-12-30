// Generated macro for rejects_truncated_session_id (function)
macro_rules! Depcrate_msgs_handshake_testrejects_truncated_session_id {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"rejects_truncated_session_id"}
// Dependencies: {}
# [test] fn rejects_truncated_session_id () { let bytes = [32 ; 32] ; let mut rd = Reader :: init (& bytes) ; assert ! (SessionId :: read (& mut rd) . is_err ()) ; }
};
}
