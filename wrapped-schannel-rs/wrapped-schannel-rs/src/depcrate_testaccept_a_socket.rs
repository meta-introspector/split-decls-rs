// Generated macro for accept_a_socket (function)
macro_rules! Depcrate_testaccept_a_socket {
() => {
// Module: crate::test
// Provides: {"accept_a_socket"}
// Dependencies: {}
# [test] fn accept_a_socket () { let cert = match localhost_cert () { Some (cert) => cert , None => return , } ; let listener = TcpListener :: bind ("127.0.0.1:0") . unwrap () ; let addr = listener . local_addr () . unwrap () ; let t = thread :: spawn (move | | { let stream = TcpStream :: connect (& addr) . unwrap () ; let creds = SchannelCred :: builder () . acquire (Direction :: Outbound) . unwrap () ; let mut stream = tls_stream :: Builder :: new () . domain ("localhost") . connect (creds , stream) . unwrap () ; stream . write_all (& [1 , 2 , 3 , 4]) . unwrap () ; stream . flush () . unwrap () ; assert_eq ! (stream . read (& mut [0 ; 1024]) . unwrap () , 4) ; stream . shutdown () . unwrap () ; }) ; let stream = listener . accept () . unwrap () . 0 ; let creds = SchannelCred :: builder () . cert (cert) . acquire (Direction :: Inbound) . unwrap () ; let mut stream = tls_stream :: Builder :: new () . accept (creds , stream) . unwrap () ; assert_eq ! (stream . read (& mut [0 ; 1024]) . unwrap () , 4) ; stream . write_all (& [1 , 2 , 3 , 4]) . unwrap () ; stream . flush () . unwrap () ; let mut buf = [0 ; 1] ; assert_eq ! (stream . read (& mut buf) . unwrap () , 0) ; t . join () . unwrap () ; }
};
}
