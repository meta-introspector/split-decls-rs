macro_rules! deps {
    () => {
        Ioctl!();
        IoctlOutput!();
        Opcode!();
        Tiocsctty!();
        Result!();
    };
}

macro_rules! impl_893 {
    () => {
        deps!();
        # [cfg (not (any (windows , target_os = "aix" , target_os = "horizon" , target_os = "redox" , target_os = "wasi")))] unsafe impl ioctl :: Ioctl for Tiocsctty { type Output = () ; const IS_MUTATING : bool = false ; fn opcode (& self) -> ioctl :: Opcode { c :: TIOCSCTTY as ioctl :: Opcode } fn as_ptr (& mut self) -> * mut c :: c_void { crate :: utils :: as_ptr (& 0_u32) as * mut c :: c_void } unsafe fn output_from_ptr (_ : ioctl :: IoctlOutput , _ : * mut c :: c_void ,) -> io :: Result < Self :: Output > { Ok (()) } }
    };
}

impl_893!()