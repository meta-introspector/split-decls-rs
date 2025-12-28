macro_rules! deps {
    () => {
        Opcode!();
        Ficlone!();
        Result!();
        Ioctl!();
        IoctlOutput!();
    };
}

macro_rules! impl_1538 {
    () => {
        deps!();
        # [cfg (all (linux_kernel , not (any (target_arch = "sparc" , target_arch = "sparc64"))))] unsafe impl ioctl :: Ioctl for Ficlone < '_ > { type Output = () ; const IS_MUTATING : bool = false ; fn opcode (& self) -> ioctl :: Opcode { c :: FICLONE as ioctl :: Opcode } fn as_ptr (& mut self) -> * mut c :: c_void { self . 0 . as_raw_fd () as * mut c :: c_void } unsafe fn output_from_ptr (_ : ioctl :: IoctlOutput , _ : * mut c :: c_void ,) -> io :: Result < Self :: Output > { Ok (()) } }
    };
}

impl_1538!()