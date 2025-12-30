// Generated macro for as_libc_timespec_ptr (function)
macro_rules! Depcrate_timespecas_libc_timespec_ptr {
() => {
// Module: crate::timespec
// Provides: {"as_libc_timespec_ptr"}
// Dependencies: {}
# [cfg (not (fix_y2038))] pub (crate) fn as_libc_timespec_ptr (timespec : & Timespec) -> * const c :: timespec { # [cfg (test)] { assert_eq_size ! (Timespec , c :: timespec) ; } crate :: utils :: as_ptr (timespec) . cast :: < c :: timespec > () }
};
}
