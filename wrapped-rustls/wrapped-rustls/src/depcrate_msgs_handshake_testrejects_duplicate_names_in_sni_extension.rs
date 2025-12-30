// Generated macro for rejects_duplicate_names_in_sni_extension (function)
macro_rules! Depcrate_msgs_handshake_testrejects_duplicate_names_in_sni_extension {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"rejects_duplicate_names_in_sni_extension"}
// Dependencies: {}
# [test] fn rejects_duplicate_names_in_sni_extension () { assert_eq ! (ClientExtensions :: read_bytes (& [0 , 14 , 0 , 0 , 0 , 10 , 0 , 8 , 0 , 0 , 1 , b'a' , 0 , 0 , 1 , b'b' ,]) . unwrap_err () , InvalidMessage :: InvalidServerName) ; }
};
}
