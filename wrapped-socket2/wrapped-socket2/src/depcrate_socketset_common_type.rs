// Generated macro for set_common_type (function)
macro_rules! Depcrate_socketset_common_type {
() => {
// Module: crate::socket
// Provides: {"set_common_type"}
// Dependencies: {}
# [doc = " Set `SOCK_CLOEXEC` and `NO_HANDLE_INHERIT` on the `ty`pe on platforms that"] # [doc = " support it."] # [inline (always)] const fn set_common_type (ty : Type) -> Type { # [cfg (any (target_os = "android" , target_os = "dragonfly" , target_os = "freebsd" , target_os = "fuchsia" , target_os = "hurd" , target_os = "illumos" , target_os = "linux" , target_os = "netbsd" , target_os = "openbsd" , target_os = "cygwin" ,))] let ty = ty . _cloexec () ; # [cfg (windows)] let ty = ty . _no_inherit () ; ty }
};
}
