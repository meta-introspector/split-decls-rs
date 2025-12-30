// Generated macro for compose_opcode (function)
macro_rules! Depcrate_ioctl_bsdcompose_opcode {
() => {
// Module: crate::ioctl::bsd
// Provides: {"compose_opcode"}
// Dependencies: {}
pub (super) const fn compose_opcode (dir : Direction , group : Opcode , num : Opcode , size : Opcode ,) -> Opcode { let dir = match dir { Direction :: None => NONE , Direction :: Read => READ , Direction :: Write => WRITE , Direction :: ReadWrite => READ | WRITE , } ; dir | num | (group << 8) | ((size & IOCPARAM_MASK) << 16) }
};
}
