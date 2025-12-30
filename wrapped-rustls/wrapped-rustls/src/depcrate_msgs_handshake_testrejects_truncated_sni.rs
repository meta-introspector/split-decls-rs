// Generated macro for rejects_truncated_sni (function)
macro_rules! Depcrate_msgs_handshake_testrejects_truncated_sni {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"rejects_truncated_sni"}
// Dependencies: {}
# [test] fn rejects_truncated_sni () { let bytes = [0 , 1 , 0] ; assert ! (ServerNamePayload :: read (& mut Reader :: init (& bytes)) . is_err ()) ; let bytes = [0 , 2 , 0 , 1] ; assert ! (ServerNamePayload :: read (& mut Reader :: init (& bytes)) . is_err ()) ; let bytes = [0 , 3 , 0 , 1 , 0] ; assert ! (ServerNamePayload :: read (& mut Reader :: init (& bytes)) . is_err ()) ; let bytes = [0 , 4 , 0 , 2 , 0 , 0] ; assert ! (ServerNamePayload :: read (& mut Reader :: init (& bytes)) . is_err ()) ; let bytes = [0 , 5 , 0 , 3 , 0 , 0 , 0] ; assert ! (ServerNamePayload :: read (& mut Reader :: init (& bytes)) . is_err ()) ; let bytes = [0 , 5 , 0 , 3 , 0 , 0 , 1] ; assert ! (ServerNamePayload :: read (& mut Reader :: init (& bytes)) . is_err ()) ; let bytes = [0 , 6 , 0 , 4 , 0 , 0 , 2 , 0x68] ; assert ! (ServerNamePayload :: read (& mut Reader :: init (& bytes)) . is_err ()) ; }
};
}
