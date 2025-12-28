macro_rules! deps {
    () => {
        Result!();
        IoctlOutput!();
        Opcode!();
    };
}

macro_rules! _ioctl {
    () => {
        deps!();
        unsafe fn _ioctl (fd : BorrowedFd < '_ > , request : Opcode , arg : * mut c :: c_void) -> Result < IoctlOutput > { crate :: backend :: io :: syscalls :: ioctl (fd , request , arg) }
    };
}

_ioctl!()