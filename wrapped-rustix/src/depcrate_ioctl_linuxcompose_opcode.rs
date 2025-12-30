// Generated macro for compose_opcode (function)
macro_rules! Depcrate_ioctl_linuxcompose_opcode {
() => {
// Module: crate::ioctl::linux
// Provides: {"compose_opcode"}
// Dependencies: {}
# [doc = " Compose an opcode from its component parts."] pub (super) const fn compose_opcode (dir : Direction , group : Opcode , num : Opcode , size : Opcode ,) -> Opcode { macro_rules ! mask_and_shift { ($ val : expr , $ shift : expr , $ mask : expr) => { { ($ val & $ mask) << $ shift } } ; } let dir = match dir { Direction :: None => NONE , Direction :: Read => READ , Direction :: Write => WRITE , Direction :: ReadWrite => READ | WRITE , } ; mask_and_shift ! (group , GROUP_SHIFT , GROUP_MASK) | mask_and_shift ! (num , NUM_SHIFT , NUM_MASK) | mask_and_shift ! (size , SIZE_SHIFT , SIZE_MASK) | mask_and_shift ! (dir , DIR_SHIFT , DIR_MASK) }
};
}
