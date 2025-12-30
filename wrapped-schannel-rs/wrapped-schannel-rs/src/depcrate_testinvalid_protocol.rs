// Generated macro for invalid_protocol (function)
macro_rules! Depcrate_testinvalid_protocol {
() => {
// Module: crate::test
// Provides: {"invalid_protocol"}
// Dependencies: {}
# [test] # [ignore] fn invalid_protocol () { let creds = SchannelCred :: builder () . enabled_protocols (& [Protocol :: Ssl3]) . acquire (Direction :: Outbound) . unwrap () ; let stream = TcpStream :: connect ("google.com:443") . unwrap () ; let err = tls_stream :: Builder :: new () . domain ("google.com") . connect (creds , stream) . err () . unwrap () ; let err = unwrap_handshake (err) ; assert_eq ! (err . raw_os_error () . unwrap () , Foundation :: SEC_E_UNSUPPORTED_FUNCTION as i32) ; }
};
}
