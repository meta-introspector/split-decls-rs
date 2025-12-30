// Generated macro for fd_set_insert (function)
macro_rules! Depcrate_event_selectfd_set_insert {
() => {
// Module: crate::event::select
// Provides: {"fd_set_insert"}
// Dependencies: {}
# [doc = " Set `fd` in the set pointed to by `fds`."] # [doc (alias = "FD_SET")] # [inline] pub fn fd_set_insert (fds : & mut [FdSetElement] , fd : RawFd) { # [cfg (not (any (windows , target_os = "wasi")))] { let fd = fd as usize ; fds [fd / BITS] . 0 |= 1 << (fd % BITS) ; } # [cfg (any (windows , target_os = "wasi"))] { let set = unsafe { & mut * fds . as_mut_ptr () . cast :: < FD_SET > () } ; let fd_count = set . fd_count ; let fd_array = & set . fd_array [.. fd_count as usize] ; if ! fd_array . contains (& (fd as _)) { let fd_array = & mut set . fd_array [.. fd_count as usize + 1] ; set . fd_count = fd_count + 1 ; fd_array [fd_count as usize] = fd as _ ; } } }
};
}
