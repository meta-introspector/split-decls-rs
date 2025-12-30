// Generated macro for option_as_libc_timespec_ptr (function)
macro_rules! Depcrate_timespecoption_as_libc_timespec_ptr {
() => {
// Module: crate::timespec
// Provides: {"option_as_libc_timespec_ptr"}
// Dependencies: {}
# [cfg (not (fix_y2038))] pub (crate) fn option_as_libc_timespec_ptr (timespec : Option < & Timespec >) -> * const c :: timespec { match timespec { None => null () , Some (timespec) => as_libc_timespec_ptr (timespec) , } }
};
}
