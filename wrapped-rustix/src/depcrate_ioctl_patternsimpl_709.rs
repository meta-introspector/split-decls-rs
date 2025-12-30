// Generated macro for impl_709 (impl)
macro_rules! Depcrate_ioctl_patternsimpl_709 {
() => {
// Module: crate::ioctl::patterns
// Provides: {"impl_709"}
// Dependencies: {}
unsafe impl < const OPCODE : Opcode > Ioctl for NoArg < OPCODE > { type Output = () ; const IS_MUTATING : bool = false ; fn opcode (& self) -> self :: Opcode { OPCODE } fn as_ptr (& mut self) -> * mut c :: c_void { core :: ptr :: null_mut () } unsafe fn output_from_ptr (_ : IoctlOutput , _ : * mut c :: c_void) -> Result < Self :: Output > { Ok (()) } }
};
}
