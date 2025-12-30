// Generated macro for impl_716 (impl)
macro_rules! Depcrate_ioctl_patternsimpl_716 {
() => {
// Module: crate::ioctl::patterns
// Provides: {"impl_716"}
// Dependencies: {}
impl < const OPCODE : Opcode , Input > Setter < OPCODE , Input > { # [doc = " Create a new pointer setter-style `ioctl` object."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = "  - `OPCODE` must provide a valid opcode."] # [doc = "  - For this opcode, `Input` must be the type that the kernel expects to"] # [doc = "    get."] # [inline] pub const unsafe fn new (input : Input) -> Self { Self { input } } }
};
}
