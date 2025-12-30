// Generated macro for _ioctl (function)
macro_rules! Depcrate_ioctl_ioctl {
() => {
// Module: crate::ioctl
// Provides: {"_ioctl"}
// Dependencies: {}
unsafe fn _ioctl (fd : BorrowedFd < '_ > , request : Opcode , arg : * mut c :: c_void) -> Result < IoctlOutput > { crate :: backend :: io :: syscalls :: ioctl (fd , request , arg) }
};
}
