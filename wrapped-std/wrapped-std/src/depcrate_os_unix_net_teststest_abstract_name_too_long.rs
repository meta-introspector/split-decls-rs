// Generated macro for test_abstract_name_too_long (function)
macro_rules! Depcrate_os_unix_net_teststest_abstract_name_too_long {
() => {
// Module: crate::os::unix::net::tests
// Provides: {"test_abstract_name_too_long"}
// Dependencies: {}
# [cfg (any (target_os = "android" , target_os = "linux" , target_os = "cygwin"))] # [test] fn test_abstract_name_too_long () { match SocketAddr :: from_abstract_name (b"abcdefghijklmnopqrstuvwxyzabcdefghijklmn\
        opqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghi\
        jklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz" ,) { Err (ref e) if e . kind () == io :: ErrorKind :: InvalidInput => { } Err (e) => panic ! ("unexpected error {e}") , Ok (_) => panic ! ("unexpected success") , } }
};
}
