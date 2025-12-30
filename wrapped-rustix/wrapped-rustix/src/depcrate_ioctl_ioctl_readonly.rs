// Generated macro for _ioctl_readonly (function)
macro_rules! Depcrate_ioctl_ioctl_readonly {
() => {
// Module: crate::ioctl
// Provides: {"_ioctl_readonly"}
// Dependencies: {}
unsafe fn _ioctl_readonly (fd : BorrowedFd < '_ > , request : Opcode , arg : * mut c :: c_void ,) -> Result < IoctlOutput > { crate :: backend :: io :: syscalls :: ioctl_readonly (fd , request , arg) }
};
}
