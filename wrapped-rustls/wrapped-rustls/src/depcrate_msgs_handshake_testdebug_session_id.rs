// Generated macro for debug_session_id (function)
macro_rules! Depcrate_msgs_handshake_testdebug_session_id {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"debug_session_id"}
// Dependencies: {}
# [test] fn debug_session_id () { let bytes = [32 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 ,] ; let mut rd = Reader :: init (& bytes) ; let sess = SessionId :: read (& mut rd) . unwrap () ; assert_eq ! ("0101010101010101010101010101010101010101010101010101010101010101" , format ! ("{sess:?}")) ; }
};
}
