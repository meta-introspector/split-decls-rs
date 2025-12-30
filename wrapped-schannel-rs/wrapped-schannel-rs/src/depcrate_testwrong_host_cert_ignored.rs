// Generated macro for wrong_host_cert_ignored (function)
macro_rules! Depcrate_testwrong_host_cert_ignored {
() => {
// Module: crate::test
// Provides: {"wrong_host_cert_ignored"}
// Dependencies: {}
# [test] fn wrong_host_cert_ignored () { let creds = SchannelCred :: builder () . acquire (Direction :: Outbound) . unwrap () ; let stream = TcpStream :: connect ("wrong.host.badssl.com:443") . unwrap () ; tls_stream :: Builder :: new () . domain ("wrong.host.badssl.com") . accept_invalid_hostnames (true) . connect (creds , stream) . unwrap () ; }
};
}
