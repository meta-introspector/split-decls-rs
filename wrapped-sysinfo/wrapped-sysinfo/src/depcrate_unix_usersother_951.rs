// Generated macro for other_951 (other)
macro_rules! Depcrate_unix_usersother_951 {
() => {
// Module: crate::unix::users
// Provides: {"other_951"}
// Dependencies: {}
# [cfg (target_os = "android")] unsafe extern "C" { fn getpwent () -> * mut libc :: passwd ; fn setpwent () ; fn endpwent () ; }
};
}
