// Generated macro for impl_711 (impl)
macro_rules! Depcrate_ioctl_patternsimpl_711 {
() => {
// Module: crate::ioctl::patterns
// Provides: {"impl_711"}
// Dependencies: {}
impl < const OPCODE : Opcode , Output > fmt :: Debug for Getter < OPCODE , Output > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("Getter") . field (& OPCODE) . finish () } }
};
}
