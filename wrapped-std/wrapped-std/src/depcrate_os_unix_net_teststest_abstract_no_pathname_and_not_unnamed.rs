// Generated macro for test_abstract_no_pathname_and_not_unnamed (function)
macro_rules! Depcrate_os_unix_net_teststest_abstract_no_pathname_and_not_unnamed {
() => {
// Module: crate::os::unix::net::tests
// Provides: {"test_abstract_no_pathname_and_not_unnamed"}
// Dependencies: {}
# [cfg (any (target_os = "android" , target_os = "linux" , target_os = "cygwin"))] # [test] fn test_abstract_no_pathname_and_not_unnamed () { let name = b"local" ; let addr = or_panic ! (SocketAddr :: from_abstract_name (name)) ; assert_eq ! (addr . as_pathname () , None) ; assert_eq ! (addr . as_abstract_name () , Some (& name [..])) ; assert_eq ! (addr . is_unnamed () , false) ; }
};
}
