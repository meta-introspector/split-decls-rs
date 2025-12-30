// Generated macro for impl_715 (impl)
macro_rules! Depcrate_ioctl_patternsimpl_715 {
() => {
// Module: crate::ioctl::patterns
// Provides: {"impl_715"}
// Dependencies: {}
impl < const OPCODE : Opcode , Input : fmt :: Debug > fmt :: Debug for Setter < OPCODE , Input > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("Setter") . field (& OPCODE) . field (& self . input) . finish () } }
};
}
