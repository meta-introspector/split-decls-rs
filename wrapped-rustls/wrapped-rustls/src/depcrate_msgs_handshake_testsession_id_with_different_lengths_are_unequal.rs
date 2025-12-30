// Generated macro for session_id_with_different_lengths_are_unequal (function)
macro_rules! Depcrate_msgs_handshake_testsession_id_with_different_lengths_are_unequal {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"session_id_with_different_lengths_are_unequal"}
// Dependencies: {}
# [test] fn session_id_with_different_lengths_are_unequal () { let a = SessionId :: read (& mut Reader :: init (& [1u8 , 1])) . unwrap () ; let b = SessionId :: read (& mut Reader :: init (& [2u8 , 1 , 2])) . unwrap () ; assert_ne ! (a , b) ; }
};
}
