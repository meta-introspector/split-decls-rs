// Generated macro for abstract_namespace_not_allowed_connect (function)
macro_rules! Depcrate_os_unix_net_testsabstract_namespace_not_allowed_connect {
() => {
// Module: crate::os::unix::net::tests
// Provides: {"abstract_namespace_not_allowed_connect"}
// Dependencies: {}
# [test] fn abstract_namespace_not_allowed_connect () { assert ! (UnixStream :: connect ("\0asdf") . is_err ()) ; }
};
}
