// Generated macro for lookup_idempotent (function)
macro_rules! Depcrate_sys_platform_version_darwin_testslookup_idempotent {
() => {
// Module: crate::sys::platform_version::darwin::tests
// Provides: {"lookup_idempotent"}
// Dependencies: {}
# [test] fn lookup_idempotent () { let version = lookup_version () ; for _ in 0 .. 10 { assert_eq ! (version , lookup_version ()) ; } }
};
}
