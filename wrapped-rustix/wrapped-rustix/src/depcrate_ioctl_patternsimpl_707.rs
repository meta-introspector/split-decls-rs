// Generated macro for impl_707 (impl)
macro_rules! Depcrate_ioctl_patternsimpl_707 {
() => {
// Module: crate::ioctl::patterns
// Provides: {"impl_707"}
// Dependencies: {}
impl < const OPCODE : Opcode > fmt :: Debug for NoArg < OPCODE > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("NoArg") . field (& OPCODE) . finish () } }
};
}
