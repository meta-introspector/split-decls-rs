// Generated macro for test (module)
macro_rules! Depcrate_identitytest {
() => {
// Module: crate::identity
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: SecIdentity ; # [test] fn identity_has_send_bound () { fn assert_send < T : Send > () { } assert_send :: < SecIdentity > () ; } }
};
}
