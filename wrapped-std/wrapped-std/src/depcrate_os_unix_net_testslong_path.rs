// Generated macro for long_path (function)
macro_rules! Depcrate_os_unix_net_testslong_path {
() => {
// Module: crate::os::unix::net::tests
// Provides: {"long_path"}
// Dependencies: {}
# [test] fn long_path () { let dir = tmpdir () ; let socket_path = dir . path () . join ("asdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfa\
                                sasdfasdfasdasdfasdfasdfadfasdfasdfasdfasdfasdf" ,) ; match UnixStream :: connect (& socket_path) { Err (ref e) if e . kind () == io :: ErrorKind :: InvalidInput => { } Err (e) => panic ! ("unexpected error {e}") , Ok (_) => panic ! ("unexpected success") , } match UnixListener :: bind (& socket_path) { Err (ref e) if e . kind () == io :: ErrorKind :: InvalidInput => { } Err (e) => panic ! ("unexpected error {e}") , Ok (_) => panic ! ("unexpected success") , } match UnixDatagram :: bind (& socket_path) { Err (ref e) if e . kind () == io :: ErrorKind :: InvalidInput => { } Err (e) => panic ! ("unexpected error {e}") , Ok (_) => panic ! ("unexpected success") , } }
};
}
