macro_rules! deps {
    () => {
        Getter!();
        Result!();
        IoctlOutput!();
        Ioctl!();
        Opcode!();
    };
}

macro_rules! impl_457 {
    () => {
        deps!();
        unsafe impl < const OPCODE : Opcode , Output > Ioctl for Getter < OPCODE , Output > { type Output = Output ; const IS_MUTATING : bool = true ; fn opcode (& self) -> self :: Opcode { OPCODE } fn as_ptr (& mut self) -> * mut c :: c_void { self . output . as_mut_ptr () . cast () } unsafe fn output_from_ptr (_ : IoctlOutput , ptr : * mut c :: c_void) -> Result < Self :: Output > { Ok (ptr . cast :: < Output > () . read ()) } }
    };
}

impl_457!()