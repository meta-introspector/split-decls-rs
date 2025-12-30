// Generated macro for or_panic (macro)
macro_rules! Depcrate_os_unix_net_testsor_panic {
() => {
// Module: crate::os::unix::net::tests
// Provides: {"or_panic"}
// Dependencies: {}
macro_rules ! or_panic { ($ e : expr) => { match $ e { Ok (e) => e , Err (e) => panic ! ("{e}") , } } ; }
};
}
