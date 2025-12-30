// Generated macro for as_libc_timespec_mut_ptr (function)
macro_rules! Depcrate_timespecas_libc_timespec_mut_ptr {
() => {
// Module: crate::timespec
// Provides: {"as_libc_timespec_mut_ptr"}
// Dependencies: {}
# [cfg (not (fix_y2038))] pub (crate) fn as_libc_timespec_mut_ptr (timespec : & mut core :: mem :: MaybeUninit < Timespec > ,) -> * mut c :: timespec { # [cfg (test)] { assert_eq_size ! (Timespec , c :: timespec) ; } timespec . as_mut_ptr () . cast :: < c :: timespec > () }
};
}
