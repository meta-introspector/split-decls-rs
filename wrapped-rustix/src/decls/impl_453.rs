macro_rules! deps {
    () => {
        Result!();
        Opcode!();
        Ioctl!();
        IoctlOutput!();
        NoArg!();
    };
}

macro_rules! impl_453 {
    () => {
        deps!();
        unsafe impl < const OPCODE : Opcode > Ioctl for NoArg < OPCODE > { type Output = () ; const IS_MUTATING : bool = false ; fn opcode (& self) -> self :: Opcode { OPCODE } fn as_ptr (& mut self) -> * mut c :: c_void { core :: ptr :: null_mut () } unsafe fn output_from_ptr (_ : IoctlOutput , _ : * mut c :: c_void) -> Result < Self :: Output > { Ok (()) } }
    };
}

impl_453!();