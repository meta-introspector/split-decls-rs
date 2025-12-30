// Generated macro for impl_723 (impl)
macro_rules! Depcrate_ioctl_patternsimpl_723 {
() => {
// Module: crate::ioctl::patterns
// Provides: {"impl_723"}
// Dependencies: {}
unsafe impl < const OPCODE : Opcode > Ioctl for IntegerSetter < OPCODE > { type Output = () ; const IS_MUTATING : bool = false ; fn opcode (& self) -> self :: Opcode { OPCODE } fn as_ptr (& mut self) -> * mut c :: c_void { self . value } unsafe fn output_from_ptr (_out : IoctlOutput , _extract_output : * mut c :: c_void ,) -> Result < Self :: Output > { Ok (()) } }
};
}
