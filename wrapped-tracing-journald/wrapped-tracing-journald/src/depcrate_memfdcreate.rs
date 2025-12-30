// Generated macro for create (function)
macro_rules! Depcrate_memfdcreate {
() => {
// Module: crate::memfd
// Provides: {"create"}
// Dependencies: {}
fn create (flags : c_uint) -> Result < File > { let fd = memfd_create_syscall (flags) ; if fd < 0 { Err (Error :: last_os_error ()) } else { Ok (unsafe { File :: from_raw_fd (fd as RawFd) }) } }
};
}
