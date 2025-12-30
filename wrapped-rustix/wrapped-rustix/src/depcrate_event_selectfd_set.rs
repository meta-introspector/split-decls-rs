// Generated macro for FD_SET (struct)
macro_rules! Depcrate_event_selectFD_SET {
() => {
// Module: crate::event::select
// Provides: {"FD_SET"}
// Dependencies: {}
# [doc = " wasi-libc's `fd_set` type. The libc bindings for it have private fields, so"] # [doc = " we redeclare it for ourselves so that we can access the fields. They're"] # [doc = " publicly exposed in wasi-libc."] # [cfg (target_os = "wasi")] # [repr (C)] struct FD_SET { # [doc = " The wasi-libc headers call this `__nfds`."] fd_count : usize , # [doc = " The wasi-libc headers call this `__fds`."] fd_array : [i32 ; c :: FD_SETSIZE] , }
};
}
