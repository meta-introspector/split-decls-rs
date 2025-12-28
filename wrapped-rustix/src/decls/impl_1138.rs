macro_rules! deps {
    () => {
        IoctlOutput!();
        Result!();
        Opcode!();
        Tiocgptpeer!();
        Ioctl!();
    };
}

macro_rules! impl_1138 {
    () => {
        deps!();
        # [cfg (target_os = "linux")] unsafe impl ioctl :: Ioctl for Tiocgptpeer { type Output = OwnedFd ; const IS_MUTATING : bool = false ; fn opcode (& self) -> ioctl :: Opcode { c :: TIOCGPTPEER as ioctl :: Opcode } fn as_ptr (& mut self) -> * mut c :: c_void { self . 0 . bits () as * mut c :: c_void } unsafe fn output_from_ptr (ret : ioctl :: IoctlOutput , _arg : * mut c :: c_void ,) -> io :: Result < Self :: Output > { Ok (OwnedFd :: from_raw_fd (ret)) } }
    };
}

impl_1138!();