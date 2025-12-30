// Generated macro for accepts_empty_session_id (function)
macro_rules! Depcrate_msgs_handshake_testaccepts_empty_session_id {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"accepts_empty_session_id"}
// Dependencies: {}
# [test] fn accepts_empty_session_id () { let bytes = [0 ; 1] ; let mut rd = Reader :: init (& bytes) ; let sess = SessionId :: read (& mut rd) . unwrap () ; println ! ("{sess:?}") ; assert ! (sess . is_empty ()) ; assert_eq ! (sess , SessionId :: empty ()) ; assert ! (! rd . any_left ()) ; }
};
}
