// Generated macro for impl_717 (impl)
macro_rules! Depcrate_ioctl_patternsimpl_717 {
() => {
// Module: crate::ioctl::patterns
// Provides: {"impl_717"}
// Dependencies: {}
unsafe impl < const OPCODE : Opcode , Input > Ioctl for Setter < OPCODE , Input > { type Output = () ; const IS_MUTATING : bool = false ; fn opcode (& self) -> self :: Opcode { OPCODE } fn as_ptr (& mut self) -> * mut c :: c_void { addr_of_mut ! (self . input) . cast :: < c :: c_void > () } unsafe fn output_from_ptr (_ : IoctlOutput , _ : * mut c :: c_void) -> Result < Self :: Output > { Ok (()) } }
};
}
