// Generated macro for impl_119 (impl)
macro_rules! Depcrate_low_level_pipeimpl_119 {
() => {
// Module: crate::low_level::pipe
// Provides: {"impl_119"}
// Dependencies: {}
impl WakeFd { # [doc = " Sets close on exec and nonblock on the inner file descriptor."] fn set_flags (& self) -> Result < () , Error > { unsafe { let flags = libc :: fcntl (self . as_raw_fd () , libc :: F_GETFL , 0) ; if flags == - 1 { return Err (Error :: last_os_error ()) ; } let flags = flags | libc :: O_NONBLOCK | libc :: O_CLOEXEC ; if libc :: fcntl (self . as_raw_fd () , libc :: F_SETFL , flags) == - 1 { return Err (Error :: last_os_error ()) ; } } Ok (()) } fn wake (& self) { wake (self . fd , self . method) ; } }
};
}
