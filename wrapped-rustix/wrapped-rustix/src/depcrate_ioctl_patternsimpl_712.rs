// Generated macro for impl_712 (impl)
macro_rules! Depcrate_ioctl_patternsimpl_712 {
() => {
// Module: crate::ioctl::patterns
// Provides: {"impl_712"}
// Dependencies: {}
impl < const OPCODE : Opcode , Output > Getter < OPCODE , Output > { # [doc = " Create a new getter-style `ioctl` object."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = "  - `OPCODE` must provide a valid opcode."] # [doc = "  - For this opcode, `Output` must be the type that the kernel expects"] # [doc = "    to write into."] # [inline] pub const unsafe fn new () -> Self { Self { output : mem :: MaybeUninit :: uninit () , } } }
};
}
