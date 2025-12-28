macro_rules! deps {
    () => {
        IoctlOutput!();
        Result!();
        Opcode!();
    };
}

macro_rules! _ioctl_readonly {
    () => {
        deps!();
        unsafe fn _ioctl_readonly (fd : BorrowedFd < '_ > , request : Opcode , arg : * mut c :: c_void ,) -> Result < IoctlOutput > { crate :: backend :: io :: syscalls :: ioctl_readonly (fd , request , arg) }
    };
}

_ioctl_readonly!();