macro_rules! deps {
    () => {
        Ioctl!();
        Opcode!();
        Result!();
        IoctlOutput!();
        Updater!();
    };
}

macro_rules! impl_464 {
    () => {
        deps!();
        unsafe impl < 'a , const OPCODE : Opcode , T > Ioctl for Updater < 'a , OPCODE , T > { type Output = () ; const IS_MUTATING : bool = true ; fn opcode (& self) -> self :: Opcode { OPCODE } fn as_ptr (& mut self) -> * mut c :: c_void { (self . value as * mut T) . cast () } unsafe fn output_from_ptr (_output : IoctlOutput , _ptr : * mut c :: c_void) -> Result < () > { Ok (()) } }
    };
}

impl_464!();